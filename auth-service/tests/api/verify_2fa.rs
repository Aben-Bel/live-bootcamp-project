use crate::helpers::TestApp;

#[tokio::test]
pub async fn verify_2_fa(){
    let app = TestApp::new().await;

    let response = app.verify_2fa(String::from("test@email.com"), String::from("password"), String::from("token")).await;

    assert_eq!(response.status().as_u16(), 200);
}