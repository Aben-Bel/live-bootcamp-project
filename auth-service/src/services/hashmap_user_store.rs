use std::collections::HashMap;

use crate::domain::{Email, Password, User, UserStoreError};
use crate::domain::UserStore;

#[derive(Default)]
pub struct HashmapUserStore {
    pub users: HashMap<Email, User>,
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
     async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        }

        self.users.insert(user.email.clone(), user);
        Ok(())
    }

    async  fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        let res = match self.users.get(&email) {
            Some(user) => Ok(user.clone()),
            None => Err(UserStoreError::UserNotFound),
        };
        return res;
    }

    async fn validate_user(&self, email: &Email, password: &Password) -> Result<(), UserStoreError> {
        match self.users.get(&email) {
            Some(user) => {
                if user.password.as_ref() == password.as_ref() {
                    return Ok(());
                } else {
                    return Err(UserStoreError::InvalidCredentials);
                }
            }

            None => Err(UserStoreError::InvalidCredentials),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let mut store = HashmapUserStore {
            users: HashMap::new(),
        };

        let user_1 = User::new(Email::parse(&("email_1@gmail.com")).unwrap(), Password::parse(&("password_1")).unwrap(), false);
        let user_2 = User::new(Email::parse(&("email_2@gmail.com")).unwrap(), Password::parse(&("password_2")).unwrap(), true);

        store.add_user(user_1.clone()).await;
        store.add_user(user_2.clone()).await;

        match store.get_user(&user_1.email).await {
            Ok(user) => assert_eq!(user_1.email, user.email),
            Err(e) => panic!("Expected Ok, got Err: {:?}", e),
        };

        match store.get_user(&user_2.email).await {
            Ok(user) => assert_eq!(user_2.email, user.email),
            Err(e) => panic!("Expected Ok, got Err: {:?}", e),
        };
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut store = HashmapUserStore {
            users: HashMap::new(),
        };

        let user_1 = User::new(Email::parse(&("email_1@gmail.com")).unwrap(), Password::parse(&("password_1")).unwrap(), false);
        let user_2 = User::new(Email::parse(&("email_2@gmail.com")).unwrap(), Password::parse(&("password_2")).unwrap(), true);

        store.add_user(user_1.clone()).await;
    
        match store.get_user(&user_1.email).await {
            Ok(user) => assert_eq!(user.email, user_1.email),
            Err(e) => panic!("Expected Ok, got Err: {:?}", e)
        }

        match store.get_user(&Email::parse(&("non_existent_email@gmail.com")).unwrap()).await {
            Ok(_user) => panic!("Expected UserNotFound error, but got Ok"),
            Err(e) => assert_eq!(UserStoreError::UserNotFound, e) 
        }
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut store = HashmapUserStore {
            users: HashMap::new(),
        };

        let user_1 = User::new(Email::parse(&("email_1@gmail.com")).unwrap(), Password::parse(&("password_1")).unwrap(), false);

        store.add_user(user_1.clone()).await;

        match store.validate_user(&user_1.email, &user_1.password).await {
            Ok(()) => (),
            Err(e) => assert_eq!(UserStoreError::UnexpectedError, e)
        } 
   }
}
