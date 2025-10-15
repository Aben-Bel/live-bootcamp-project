use std::collections::HashMap;

use crate::domain::User;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

// TODO: Create a new struct called `HashmapUserStore` containing a `users` field
// which stores a `HashMap`` of email `String`s mapped to `User` objects.
// Derive the `Default` trait for `HashmapUserStore`.

#[derive(Default)]
pub struct HashmapUserStore {
    pub users: HashMap<String, User>,
}

impl HashmapUserStore {
    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        // Return `UserStoreError::UserAlreadyExists` if the user already exists,
        // otherwise insert the user into the hashmap and return `Ok(())`.
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        }

        self.users.insert(user.email.clone(), user);
        Ok(())
    }

    // TODO: Implement a public method called `get_user`, which takes an
    // immutable reference to self and an email string slice as arguments.
    // This function should return a `Result` type containing either a
    // `User` object or a `UserStoreError`.
    // Return `UserStoreError::UserNotFound` if the user can not be found.
    pub fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        let res = match self.users.get(email) {
            Some(user) => Ok(user.clone()),
            None => Err(UserStoreError::UserNotFound),
        };
        return res;
    }

    // TODO: Implement a public method called `validate_user`, which takes an
    // immutable reference to self, an email string slice, and a password string slice
    // as arguments. `validate_user` should return a `Result` type containing either a
    // unit type `()` if the email/password passed in match an existing user, or a `UserStoreError`.
    // Return `UserStoreError::UserNotFound` if the user can not be found.
    // Return `UserStoreError::InvalidCredentials` if the password is incorrect.
    pub fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        match self.users.get(email) {
            Some(user) => {
                if user.password == password {
                    return Ok(());
                } else {
                    return Err(UserStoreError::InvalidCredentials);
                }
            }

            None => Err(UserStoreError::InvalidCredentials),
        }
    }
}

// TODO: Add unit tests for your `HashmapUserStore` implementation
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let mut store = HashmapUserStore {
            users: HashMap::new(),
        };

        let user_1 = User::new(String::from("email_1"), String::from("password_1"), false);
        let user_2 = User::new(String::from("email_2"), String::from("password_2"), true);

        store.add_user(user_1.clone());
        store.add_user(user_2.clone());

        match store.get_user(&user_1.email) {
            Ok(user) => assert_eq!(user_1.email, user.email),
            Err(e) => panic!("Expected Ok, got Err: {:?}", e),
        };

        match store.get_user(&user_2.email) {
            Ok(user) => assert_eq!(user_2.email, user.email),
            Err(e) => panic!("Expected Ok, got Err: {:?}", e),
        };
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut store = HashmapUserStore {
            users: HashMap::new(),
        };

        let user_1 = User::new(String::from("email_1"), String::from("password_1"), false);
        let user_2 = User::new(String::from("email_2"), String::from("password_2"), true);

        store.add_user(user_1.clone());
    
        match store.get_user(&user_1.email) {
            Ok(user) => assert_eq!(user.email, user_1.email),
            Err(e) => panic!("Expected Ok, got Err: {:?}", e)
        }

        match store.get_user(&String::from("non_existent_email")) {
            Ok(_user) => panic!("Expected UserNotFound error, but got Ok"),
            Err(e) => assert_eq!(UserStoreError::UserNotFound, e) 
        }
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut store = HashmapUserStore {
            users: HashMap::new(),
        };

        let user_1 = User::new(String::from("email_1"), String::from("password_1"), false);

        store.add_user(user_1.clone());

        match store.validate_user(&user_1.email, &user_1.password) {
            Ok(()) => (),
            Err(e) => assert_eq!(UserStoreError::UnexpectedError, e)
        } 
   }
}
