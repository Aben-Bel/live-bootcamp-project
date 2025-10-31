use auth_service::Application;
use auth_service::domain::Email;
use auth_service::routes::TwoFactorAuthResponse;
use auth_service::{utils::constants::JWT_COOKIE_NAME, ErrorResponse};
use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
pub async fn login(){
    let app = TestApp::new().await;

    app.post_signup(&serde_json::json!({
            "email":String::from("test@email.com"),
            "password": "password123",
            "requires_2fa": true
        })).await;

    let response = app.login(String::from("test@email.com"), String::from("password1234")).await;

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
pub async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let test_cases = [
        serde_json::json!({
            "password": "password123",
        }),
        serde_json::json!({
            "email":"email",
        })
    ];
    
    for test in &test_cases {
        let response = app.post_login(test).await;
               assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test
        ); 
    }
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;

    let test_cases = [
        serde_json::json!({
            "email":"emailemail.com",
            "password": "password123",
        }),
        serde_json::json!({
            "password":"pass",
            "email":"email@email.com",
        })
    ];
    
    for test in &test_cases {
        let response = app.post_login(test).await;
               assert_eq!(
            response.status().as_u16(),
            400,
            "Failed for input: {:?}",
            test
        ); 
    }
}

#[tokio::test]
async fn should_return_200_if_valid_credentials_and_2fa_disabled() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires_2fa": false
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());
}

#[tokio::test]
async fn should_return_206_if_valid_credentials_and_2fa_enabled() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires_2fa": true 
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 206);

    let json_body = response
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");

    assert_eq!(json_body.message, "2FA required".to_owned());

    // TODO: assert that `json_body.login_attempt_id` is stored inside `app.two_fa_code_store`
    let store = app.two_fa_code_store.write().await;

    let result = store.get_code(&(Email::parse(&random_email)).unwrap()).await.unwrap();
    assert_eq!(result.0.as_ref(), json_body.login_attempt_id)
}