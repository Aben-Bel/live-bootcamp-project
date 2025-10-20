use std::ops::Deref;

use validator::{validate_email, validate_length};

#[derive(PartialEq, Debug, Clone, Eq, Hash)]
pub struct Email(String);

impl Email {
    pub fn parse(email: &str) -> Result<Email, String> {
        if validate_email(email) {
            return Ok(Email(email.to_string()));
        }
        Err(String::from("Invalid Email"))
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(PartialEq, Debug, Clone)]
pub struct Password(String);

impl Password {
    pub fn parse(password: &str) -> Result<Password, String> {
        if validate_length(password, Some(8), None, None) {
            return Ok(Password(password.to_string()));
        }
        Err(String::from("Invalid Password"))
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_email_parse() {
        assert!(Email::parse("hello").is_err());
        assert_eq!(
            Email::parse("hello@gmail.com"),
            Ok(Email(String::from("hello@gmail.com")))
        );
    }
}
// The User struct should contain 3 fields. email, which is a String;
// password, which is also a String; and requires_2fa, which is a boolean.
#[derive(Clone, Debug, PartialEq)]
pub struct User {
    pub email: Email,
    pub password: Password,
    pub requires_2fa: bool,
}

impl User {
    pub fn new(email: Email, password: Password, requires_2fa: bool) -> Self {
        Self {
            email,
            password,
            requires_2fa,
        }
    }
}
