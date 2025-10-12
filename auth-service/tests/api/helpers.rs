use auth_service::Application;
use reqwest;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub http_client: reqwest::Client,
}

impl TestApp {
    pub async fn new() -> Self {
        let app = Application::build("127.0.0.1:0")
            .await
            .expect("Failed to build app");

        let address = format!("http://{}", app.address.clone());

        // Run the auth service in a separate async task
        // to avoid blocking the main test thread. 
        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let http_client = reqwest::Client::new(); // Create a Reqwest http client instance

        // Create new `TestApp` instance and return it
        TestApp {address, http_client}
    }

    pub async fn get_root(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    // TODO: Implement helper functions for all other routes (signup, login, logout, verify-2fa, and verify-token)
    pub async fn signup(&self, email : String, password :  String, requires2fa : bool )-> reqwest::Response {
        #[derive(serde::Serialize)]
        struct SignupData {
            email: String,
            password: String,
            requires2fa: bool,
        }
        
        let sign_up_data = SignupData {
            email, password, requires2fa
        };
        println!("{}", format!("{}{}",&self.address, "/signup"));
        let response = self.http_client
            .post(format!("{}{}",&self.address, "/signup"))
            .json(&sign_up_data)
            .send()
            .await
            .expect("Failed to execute request");

        response   
    }

    pub async fn login(&self, email : String, password :  String)-> reqwest::Response {

        #[derive(serde::Serialize)]
        struct LoginData {
            email: String,
            password: String,
        }
        
        let login_data = LoginData {
            email, password
        };
        
        let response = self.http_client
            .post(format!("{}{}",&self.address, "/login"))
            .json(&login_data)
            .send()
            .await
            .expect("Failed to execute request");

        response
    }

    pub async fn logout(&self, email : String)-> reqwest::Response {
        #[derive(serde::Serialize)]
        struct LogoutData {
            email: String
        }


       let response = self.http_client
            .post(format!("{}{}",&self.address, "/logout"))
            .json(&LogoutData { email })
            .send()
            .await
            .expect("Failed to execute request");

        response
    }

    pub async fn verify_2fa(&self, email : String, login_attempt_id : String, two_fa_code : String)-> reqwest::Response {
        #[derive(serde::Serialize)]
        struct Verify2FA {
            email: String,
            login_attempt_id: String,
            two_fa_code: String,
        }
        
        let verify_2_fa = Verify2FA {
            email, login_attempt_id, two_fa_code
        };
        
        let response = self.http_client
            .post(format!("{}{}",&self.address, "/verify_2_fa"))
            .json(&verify_2_fa)
            .send()
            .await
            .expect("Failed to execute request");

        response
    }

    pub async fn verify_token(&self, token : String)-> reqwest::Response {
        #[derive(serde::Serialize)]
        struct VerifyToken {
            token: String
        }

        let response = self.http_client
                .post(format!("{}{}",&self.address, "/verify_token"))
                .json(&VerifyToken { token })
                .send()
                .await
                .expect("Failed to execute request");

            response
        }

    pub async fn post_signup<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/signup", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

pub fn get_random_email() -> String {
    format!("{}@example.com", Uuid::new_v4())
}