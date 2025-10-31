use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, LoginAttemptId, Password, TwoFACode},
    utils::auth::generate_auth_cookie,
};

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

pub async fn login(
    state: State<AppState>,
    jar: CookieJar,
    Json(login_request): Json<LoginRequest>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let LoginRequest { email, password } = login_request;

    let res_email = match Email::parse(&email) {
        Ok(e) => e,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let res_password = match Password::parse(&password) {
        Ok(p) => p,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let store = state.user_store.write().await;

    if let Err(_) = store.validate_user(&res_email, &res_password).await {
        return (jar, Err(AuthAPIError::IncorrectCredentials));
    }

    let user = match store.get_user(&res_email).await {
        Ok(user) => user,
        Err(_) => return (jar, Err(AuthAPIError::IncorrectCredentials)),
    };

    // Handle request based on user's 2FA configuration
    match user.requires_2fa {
        true => handle_2fa(&user.email, &state, jar).await,
        false => handle_no_2fa(&user.email, jar).await,
    }
}

async fn handle_2fa(
    email: &Email,
    state: &AppState,
    jar: CookieJar,
) -> (
    CookieJar,
    Result<(StatusCode, Json<LoginResponse>), AuthAPIError>,
) {
    let login_attempt_id = LoginAttemptId::default();
    let two_fa_code = TwoFACode::default();

    if let Err(_) = state
        .two_fa_code_store
        .write()
        .await
        .add_code(email.clone(), login_attempt_id.clone(), two_fa_code.clone())
        .await
    {
        return (jar, Err(AuthAPIError::UnexpectedError));
    }

    let email_client = state.email_client.read().await;
    if let Err(_) = email_client.send_email(&email, &"Verification Code".to_owned(), two_fa_code.as_ref()).await {
        return (jar, Err(AuthAPIError::UnexpectedError));
    }
    
    return (
        jar,
        Ok((
            StatusCode::PARTIAL_CONTENT,
            Json(LoginResponse::TwoFactorAuth(TwoFactorAuthResponse {
                message: "2FA required".to_owned(),
                login_attempt_id: login_attempt_id.as_ref().to_string(),
            })),
        )),
    );
}

async fn handle_no_2fa(
    email: &Email,
    jar: CookieJar,
) -> (
    CookieJar,
    Result<(StatusCode, Json<LoginResponse>), AuthAPIError>,
) {
    match generate_auth_cookie(&email) {
        Ok(cookie) => (
            jar.add(cookie),
            Ok((StatusCode::OK, Json(LoginResponse::RegularAuth))),
        ),
        Err(_) => (jar, Err(AuthAPIError::UnexpectedError)),
    }
}

// The login route can return 2 possible success responses.
// This enum models each response!
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum LoginResponse {
    RegularAuth,
    TwoFactorAuth(TwoFactorAuthResponse),
}

// If a user requires 2FA, this JSON body should be returned!
#[derive(Debug, Serialize, Deserialize)]
pub struct TwoFactorAuthResponse {
    pub message: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
}
