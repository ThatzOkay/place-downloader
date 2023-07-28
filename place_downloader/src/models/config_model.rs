use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub accounts: Vec<Account>
}

#[derive(Debug, Serialize, Deserialize)]
#[derive(Clone)]
pub struct Account {
    pub username: String,
    pub password: Option<String>,
    pub jwt_token: Option<String>,
}

impl AppConfig {
    pub fn new(accounts: Vec<Account>) -> Self {
        AppConfig {
            accounts,
        }
    }

    pub fn new_empty() -> Self {
        AppConfig { accounts: Vec::new() }
    }
}

impl Account {
    pub fn with_credentials(username: String, password: String) -> Self {
        Account {
            username,
            password: Some(password),
            jwt_token: None,
        }
    }

    pub fn with_token(username: String, jwt_token: String) -> Self {
        Account {
            username,
            password: None,
            jwt_token: Some(jwt_token),
        }
    }
}
