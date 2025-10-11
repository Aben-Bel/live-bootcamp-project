use crate::helpers::TestApp;

#[tokio::test]
pub async fn verify_token(){
    let app = TestApp::new().await;

    let response = app.verify_token(String::from("token")).await;

    assert_eq!(response.status().as_u16(), 200);
}