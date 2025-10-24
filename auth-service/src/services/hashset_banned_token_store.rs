use std::collections::HashSet;

use crate::domain::{BannedTokenStore, BannedTokenStoreError};


#[derive(Default, Debug)]
pub struct HashsetBannedTokenStore {
    pub tokens: HashSet<String>,
}

#[async_trait::async_trait]
impl  BannedTokenStore for HashsetBannedTokenStore {
    async fn add_token(&mut self, token: String) -> Result<(), BannedTokenStoreError> {
        if self.tokens.contains(&token) {
            Err(BannedTokenStoreError::UnexpectedError)
        } else {
            self.tokens.insert(token);
            Ok(())
        }
    }

    async fn is_banned_token(&self, token: &str) -> Result<bool, BannedTokenStoreError> {
        if self.tokens.contains(token) {
            return Ok(true);
        } else {
            return Ok(false);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_token() {
        let mut store = HashsetBannedTokenStore {
            tokens: HashSet::new(),
        };

        store.add_token("token1".to_owned()).await;
        assert!(store.tokens.contains("token1"));

    }

    #[tokio::test]
    async fn test_is_banned_token() {
        let mut store = HashsetBannedTokenStore {
            tokens: HashSet::new(),
        };

        store.add_token("token1".to_owned()).await;
        assert!(store.tokens.contains("token1")); 

        assert!(store.is_banned_token("token1").await.unwrap());
    }
}