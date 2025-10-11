use crate::helpers::TestApp;

#[tokio::test]
pub async fn logout(){
    let app = TestApp::new().await;

    let response = app.logout(String::from("test@email.com")).await;

    assert_eq!(response.status().as_u16(), 200);
}