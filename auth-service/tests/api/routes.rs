use crate::helpers::TestApp;

// Tokio's test macro is used to run the test in an async environment
#[tokio::test]
async fn root_returns_auth_ui() {
    let app = TestApp::new().await;

    let response = app.get_root().await;

    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}

// TODO: Implement tests for all other routes (signup, login, logout, verify-2fa, and verify-token)
// For now, simply assert that each route returns a 200 HTTP status code.
#[tokio::test]
async fn sign_up(){
    let app = TestApp::new().await;

    let response = app.signup(String::from("test@email.com"), String::from("password"), false).await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn login(){
    let app = TestApp::new().await;

    let response = app.login(String::from("test@email.com"), String::from("password")).await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn logout(){
    let app = TestApp::new().await;

    let response = app.logout(String::from("test@email.com")).await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn verify_2_fa(){
    let app = TestApp::new().await;

    let response = app.verify_2fa(String::from("test@email.com"), String::from("password"), String::from("token")).await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn verify_token(){
    let app = TestApp::new().await;

    let response = app.verify_token(String::from("token")).await;

    assert_eq!(response.status().as_u16(), 200);
}