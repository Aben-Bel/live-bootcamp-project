use auth_service::{domain::Email, routes::TwoFactorAuthResponse, utils::constants::JWT_COOKIE_NAME};
use axum::http::response;
use reqwest::Url;

use crate::helpers::{TestApp, get_random_email};

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let verify_2fa_body = serde_json::json!({
        "email": random_email,
        "loginAttemptId": "id",
    });

    let response = app.post_verify_2fa(&verify_2fa_body).await;

    assert_eq!(response.status().as_u16(), 422);

}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let verify_2fa_body = serde_json::json!({
        "email": random_email,
        "loginAttemptId": "id",
        "2FACode":"123456"
    });

    let response = app.post_verify_2fa(&verify_2fa_body).await;

    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
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
        "password": "wrong_password",
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn should_return_401_if_old_code() {
    // Call login twice. Then, attempt to call verify-fa with the 2FA code from the first login requet. This should fail. 
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

    let two_factor_auth_response : TwoFactorAuthResponse = response.json().await.unwrap();
    let first_attempt_id = two_factor_auth_response.login_attempt_id;

    let email_parsed = Email::parse(&random_email).unwrap();
    let first_code = {
        let store = app.two_fa_code_store.read().await;
        let (_, code) = store.get_code(&email_parsed).await.unwrap();
        code.as_ref().to_string()
    };

    let response = app.post_login(&login_body).await;

    let two_factor_auth_response : TwoFactorAuthResponse = response.json().await.unwrap();
    let second_attempt_id = two_factor_auth_response.login_attempt_id;

    let second_code = {
        let store = app.two_fa_code_store.read().await;
        let (_, code) = store.get_code(&email_parsed).await.unwrap();
        code.as_ref().to_string()
    };

    let response_from_verify_2fa = app.post_verify_2fa(&serde_json::json!({
       "email": random_email,
        "loginAttemptId": first_attempt_id,
        "2FACode": first_code
    })).await;

    assert_eq!(response_from_verify_2fa.status().as_u16(), 401);

    let response_from_verify_2fa = app.post_verify_2fa(&serde_json::json!({
       "email": random_email,
        "loginAttemptId": second_attempt_id,
        "2FACode": second_code
    })).await;

    assert_eq!(response_from_verify_2fa.status().as_u16(), 200); 

}

#[tokio::test]
async fn should_return_200_if_correct_code() {
    // Make sure to assert the auth cookie gets set
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

    let two_factor_auth_response : TwoFactorAuthResponse = response.json().await.unwrap();
    let first_attempt_id = two_factor_auth_response.login_attempt_id;

    let email_parsed = Email::parse(&random_email).unwrap();
    let first_code = {
        let store = app.two_fa_code_store.read().await;
        let (_, code) = store.get_code(&email_parsed).await.unwrap();
        code.as_ref().to_string()
    };

    let response_from_verify_2fa = app.post_verify_2fa(&serde_json::json!({
       "email": random_email,
        "loginAttemptId": first_attempt_id,
        "2FACode": first_code
    })).await;

    assert_eq!(response_from_verify_2fa.status().as_u16(), 200);

    let auth_cookie = response_from_verify_2fa
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty()); 

}

#[tokio::test]
async fn should_return_401_if_same_code_twice() {    
        // Make sure to assert the auth cookie gets set
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

    let two_factor_auth_response : TwoFactorAuthResponse = response.json().await.unwrap();
    let first_attempt_id = two_factor_auth_response.login_attempt_id;

    let email_parsed = Email::parse(&random_email).unwrap();
    let first_code = {
        let store = app.two_fa_code_store.read().await;
        let (_, code) = store.get_code(&email_parsed).await.unwrap();
        code.as_ref().to_string()
    };

    let response_from_verify_2fa = app.post_verify_2fa(&serde_json::json!({
       "email": random_email,
        "loginAttemptId": first_attempt_id,
        "2FACode": first_code
    })).await;

    assert_eq!(response_from_verify_2fa.status().as_u16(), 200);

    let auth_cookie = response_from_verify_2fa
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty()); 

    let response_from_verify_2fa = app.post_verify_2fa(&serde_json::json!({
       "email": random_email,
        "loginAttemptId": first_attempt_id,
        "2FACode": first_code
    })).await;

    assert_eq!(response_from_verify_2fa.status().as_u16(), 401);
}