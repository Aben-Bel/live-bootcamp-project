use std::{collections::HashMap, sync::Arc};

use auth_service::{app_state::AppState, services::hashmap_user_store::HashmapUserStore, Application};
use axum::{response::Html };
use tokio::sync::RwLock;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let user_store = Arc::new(RwLock::new(HashmapUserStore {
        users : HashMap::new()
    })); 

    let app_state = AppState {
        user_store : user_store
    }; 

    let app = Application::build(app_state, "0.0.0.0:3000")
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}

async fn hello_handler() -> Html<&'static str> {
    // TODO: Update this to a custom message!
    Html("<h1>Hello, World!</h1>")
}
