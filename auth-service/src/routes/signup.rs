

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, Password, User, UserStoreError}, 
};
use crate::domain::UserStore;

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
) -> Result<impl IntoResponse, AuthAPIError> {
    // Create a new `User` instance using data in the `request`
    let email = Email::parse(&request.email);
    let password = Password::parse(&request.password);

    if email.is_err() || password.is_err() {
        return Err(AuthAPIError::BadRequest);
    }
    
    let user = User {
        email : email.unwrap(),
        password: password.unwrap(),
        requires_2fa: request.requires_2fa
    };

    let mut user_store = state.user_store.write().await;

    match user_store.get_user(&user.email).await {
        Ok(user) => return Err(AuthAPIError::UserAlreadyExists),
        Err(UserStoreError::UserNotFound) =>  {
            user_store.add_user(user).await.unwrap();
            let response = Json(SignupResponse {
                message: "User created successfully!".to_string(),
            });

            Ok((StatusCode::CREATED, response))
        },
        _ => Err(AuthAPIError::UnexpectedError)
    }
}