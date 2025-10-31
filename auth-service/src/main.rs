use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use auth_service::{
    Application, app_state::AppState, services::{
        hashmap_two_fa_code_store::HashmapTwoFACodeStore, hashmap_user_store::HashmapUserStore,
        hashset_banned_token_store::HashsetBannedTokenStore, mock_email_client::MockEmailClient,
    }, utils::constants::prod
};
use axum::response::Html;
use tokio::sync::RwLock;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let user_store = Arc::new(RwLock::new(HashmapUserStore {
        users: HashMap::new(),
    }));
    let banned_token_store = Arc::new(RwLock::new(HashsetBannedTokenStore {
        tokens: HashSet::new(),
    }));
    let two_fa_code_store = Arc::new(RwLock::new(HashmapTwoFACodeStore {
        codes: HashMap::new(),
    }));
    let email_client =  Arc::new(RwLock::new(MockEmailClient));

    let app_state = AppState {
        user_store: user_store,
        banned_token_store: banned_token_store,
        two_fa_code_store,
        email_client
    };

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}

async fn hello_handler() -> Html<&'static str> {
    // TODO: Update this to a custom message!
    Html("<h1>Hello, World!</h1>")
}
