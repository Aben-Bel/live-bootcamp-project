use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;

use crate::{
    app_state::AppState, domain::AuthAPIError, utils::{auth::validate_token, constants::JWT_COOKIE_NAME}
};

pub async fn logout(
    app_state: State<AppState>,
    jar: CookieJar
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let cookie = match jar.get(JWT_COOKIE_NAME) {
        Some(cookie) => cookie,
        None => return (jar, Err(AuthAPIError::MissingToken)),
    };

    let token = cookie.value().to_owned();

    let mut banned_token_store = app_state.banned_token_store.write().await;

    let jar = jar.remove(JWT_COOKIE_NAME);

    match validate_token(&token,Some(&mut *banned_token_store)).await {
        Ok(_claim) => {
            banned_token_store.add_token(token.to_string()).await.unwrap();
            return (jar, Ok(StatusCode::OK));
        },
        Err(error) => {
            eprintln!("Token validation failed: {:?}", error);
            (jar, Err(AuthAPIError::InvalidToken))
        }
    }
}
