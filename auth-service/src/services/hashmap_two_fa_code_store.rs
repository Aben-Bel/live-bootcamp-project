use std::collections::HashMap;

use crate::domain::{
    Email, LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError
};

#[derive(Default)]
pub struct HashmapTwoFACodeStore {
    pub codes: HashMap<Email, (LoginAttemptId, TwoFACode)>,
}

// TODO: implement TwoFACodeStore for HashmapTwoFACodeStore
#[async_trait::async_trait]
impl TwoFACodeStore for HashmapTwoFACodeStore {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError> {
        self.codes.insert(email, (login_attempt_id, code));
        Ok(())
    }

    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError>{
        match self.codes.get(&email) {
            Some((_login_attempt_id, _two_fa_code)) => {
                self.codes.remove(&email);
                Ok(())
            },
            None => {
                return Err(TwoFACodeStoreError::UnexpectedError);
            }
        }
    }

    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError>{
       match self.codes.get(&email) {
       Some(result) => {
            return Ok(result.clone());
       },
       None => {
            return Err(TwoFACodeStoreError::LoginAttemptIdNotFound);
       } 
       } 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_code_success() {
        let mut store = HashmapTwoFACodeStore::default();
        let email = Email::parse(&"test@example.com".to_string()).unwrap();
        let login_attempt_id = LoginAttemptId::default();
        let code = TwoFACode::default();

        let result = store.add_code(email.clone(), login_attempt_id.clone(), code.clone()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_add_code_overwrites_existing() {
        let mut store = HashmapTwoFACodeStore::default();
        let email = Email::parse(&"test@example.com".to_string()).unwrap();
        let login_attempt_id_1 = LoginAttemptId::default();
        let code_1 = TwoFACode::default();

        // Add first code
        store.add_code(email.clone(), login_attempt_id_1.clone(), code_1.clone()).await.unwrap();

        // Add second code with different values
        let login_attempt_id_2 = LoginAttemptId::default();
        let code_2 = TwoFACode::default();
        let result = store.add_code(email.clone(), login_attempt_id_2.clone(), code_2.clone()).await;
        
        assert!(result.is_ok());

        // Verify the second code is stored, not the first
        let (stored_id, stored_code) = store.get_code(&email).await.unwrap();
        assert_eq!(stored_id, login_attempt_id_2);
        assert_eq!(stored_code, code_2);
    }

    #[tokio::test]
    async fn test_get_code_success() {
        let mut store = HashmapTwoFACodeStore::default();
        let email = Email::parse(&"test@example.com".to_string()).unwrap();
        let login_attempt_id = LoginAttemptId::default();
        let code = TwoFACode::default();

        store.add_code(email.clone(), login_attempt_id.clone(), code.clone()).await.unwrap();

        let result = store.get_code(&email).await;
        assert!(result.is_ok());
        let (stored_id, stored_code) = result.unwrap();
        assert_eq!(stored_id, login_attempt_id);
        assert_eq!(stored_code, code);
    }

    #[tokio::test]
    async fn test_get_code_not_found() {
        let store = HashmapTwoFACodeStore::default();
        let email = Email::parse(&"nonexistent@example.com".to_string()).unwrap();

        let result = store.get_code(&email).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), TwoFACodeStoreError::LoginAttemptIdNotFound);
    }

    #[tokio::test]
    async fn test_remove_code_success() {
        let mut store = HashmapTwoFACodeStore::default();
        let email = Email::parse(&"test@example.com".to_string()).unwrap();
        let login_attempt_id = LoginAttemptId::default();
        let code = TwoFACode::default();

        store.add_code(email.clone(), login_attempt_id, code).await.unwrap();

        let result = store.remove_code(&email).await;
        assert!(result.is_ok());

        // Verify it's actually removed
        let get_result = store.get_code(&email).await;
        assert!(get_result.is_err());
    }

    #[tokio::test]
    async fn test_remove_code_not_found() {
        let mut store = HashmapTwoFACodeStore::default();
        let email = Email::parse(&"nonexistent@example.com".to_string()).unwrap();

        let result = store.remove_code(&email).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), TwoFACodeStoreError::UnexpectedError);
    }

    #[tokio::test]
    async fn test_multiple_users() {
        let mut store = HashmapTwoFACodeStore::default();
        
        let email1 = Email::parse(&"user1@example.com".to_string()).unwrap();
        let email2 = Email::parse(&"user2@example.com".to_string()).unwrap();
        
        let id1 = LoginAttemptId::default();
        let id2 = LoginAttemptId::default();
        
        let code1 = TwoFACode::default();
        let code2 = TwoFACode::default();

        // Add codes for both users
        store.add_code(email1.clone(), id1.clone(), code1.clone()).await.unwrap();
        store.add_code(email2.clone(), id2.clone(), code2.clone()).await.unwrap();

        // Verify both are stored correctly
        let (stored_id1, stored_code1) = store.get_code(&email1).await.unwrap();
        let (stored_id2, stored_code2) = store.get_code(&email2).await.unwrap();

        assert_eq!(stored_id1, id1);
        assert_eq!(stored_code1, code1);
        assert_eq!(stored_id2, id2);
        assert_eq!(stored_code2, code2);
    }
}