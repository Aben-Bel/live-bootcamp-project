use crate::helpers::TestApp;

#[tokio::test]
pub async fn login(){
    let app = TestApp::new().await;

    let response = app.login(String::from("test@email.com"), String::from("password")).await;

    assert_eq!(response.status().as_u16(), 200);
}