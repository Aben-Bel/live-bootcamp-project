use crate::helpers::TestApp;

#[tokio::test]
pub async fn sign_up(){
    let app = TestApp::new().await;

    let response = app.signup(String::from("test@email.com"), String::from("password"), false).await;

    assert_eq!(response.status().as_u16(), 200);
}