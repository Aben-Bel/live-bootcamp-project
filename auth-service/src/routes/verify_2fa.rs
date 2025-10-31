use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, LoginAttemptId, TwoFACode},
    utils::auth::generate_auth_cookie,
};

#[derive(Serialize, Deserialize)]
pub struct Verify2FARequest {
    email: String,
    #[serde(rename = "loginAttemptId")]
    login_attempt_id: String,
    #[serde(rename = "2FACode")]
    two_fa_code: String,
}

pub async fn verify_2fa(
    jar: CookieJar,
    State(state): State<AppState>,
    Json(request): Json<Verify2FARequest>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let email = match Email::parse(&request.email) {
        Ok(e) => e,
        Err(_) => return (jar, Err(AuthAPIError::BadRequest)),
    };
    let login_attempt_id = match LoginAttemptId::parse(request.login_attempt_id) {
        Ok(id) => id,
        Err(_) => return (jar, Err(AuthAPIError::BadRequest)),
    };

    let two_fa_code = match TwoFACode::parse(request.two_fa_code) {
        Ok(code) => code,
        Err(_) => return (jar, Err(AuthAPIError::BadRequest)),
    };

    let mut two_fa_code_store = state.two_fa_code_store.write().await;
    let code_tuple = match two_fa_code_store.get_code(&email).await {
        Ok(tuple) => tuple,
        Err(_) => return (jar, Err(AuthAPIError::IncorrectCredentials)),
    };

    let (attempt_id, code): (LoginAttemptId, TwoFACode) = code_tuple;

    if login_attempt_id != attempt_id || two_fa_code != code {
        return (jar, Err(AuthAPIError::IncorrectCredentials));
    }

    match generate_auth_cookie(&email) {
        Ok(cookie) => {
            if let Err(_) = two_fa_code_store
                .remove_code(&email)
                .await
            {
                return (jar, Err(AuthAPIError::UnexpectedError));
            }

            (jar.add(cookie), Ok(StatusCode::OK))
        }
        Err(_) => (jar, Err(AuthAPIError::UnexpectedError)),
    }
}

