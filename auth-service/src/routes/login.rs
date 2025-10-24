use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, Password}, utils::auth::generate_auth_cookie,
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
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>){
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
    
    match generate_auth_cookie(&res_email) {
        Ok(cookie) => (jar.add(cookie), Ok(StatusCode::OK.into_response())),
        Err(_) => (jar, Err(AuthAPIError::UnexpectedError)),
    }
}
