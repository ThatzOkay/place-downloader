use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub accounts: Vec<Account>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub username: Option<String>,
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
    fn with_credentials(username: String, password: String) -> Self {
        Account {
            username: Some(username),
            password: Some(password),
            jwt_token: None,
        }
    }

    fn with_token(username: String, jwt_token: String) -> Self {
        Account {
            username: Some(username),
            password: None,
            jwt_token: Some(jwt_token),
        }
    }
}
