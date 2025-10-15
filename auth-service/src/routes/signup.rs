

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use crate::{app_state::AppState, domain::User};
#[derive(Deserialize)]
pub struct SignupRequest{
    pub email : String,
    pub password : String,
    pub requires_2fa: bool,
}


#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SignupResponse{
    pub message : String
}


pub async fn signup(
    // TODO: Use Axum's state extractor to pass in AppState
    state : State<AppState>,
    Json(request): Json<SignupRequest>,
) -> impl IntoResponse {
    // Create a new `User` instance using data in the `request`
    let user = User {
        email : request.email,
        password: request.password,
        requires_2fa: request.requires_2fa
    };

    let mut user_store = state.user_store.write().await;

    // TODO: Add `user` to the `user_store`. Simply unwrap the returned `Result` enum type for now.
    user_store.add_user(user).unwrap();
    let response = Json(SignupResponse {
        message: "User created successfully!".to_string(),
    });

    (StatusCode::CREATED, response)
}