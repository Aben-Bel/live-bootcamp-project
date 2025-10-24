use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

use crate::{app_state::AppState, domain::AuthAPIError, utils::auth::validate_token};


#[derive(Deserialize)]
pub struct VerifyToken {
    token : String
}


pub async fn verify_token(
    state : State<AppState>,
    Json(request) : Json<VerifyToken>
) -> 
Result<impl IntoResponse, AuthAPIError> {
    let VerifyToken { token } = request;
    let banned_token_store = state.banned_token_store.clone();

    match validate_token(&token, banned_token_store).await {
        Ok(_) => {
            return Ok(StatusCode::OK);
        },
        Err(_error) => {
            return Err(AuthAPIError::InvalidToken);
        }
    }
}