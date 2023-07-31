use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
use colored::Colorize;
use log;
use rand::Rng;
use reqwest::{
    cookie::Jar,
    header::{HeaderName, HeaderValue},
    Client, ClientBuilder,
};
use serde_json::Value;
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use crate::models::token_model::Token;

pub struct RedditManager;

// TODO: Move to config
pub const REDDIT_URL: &str = "https://www.reddit.com/";
pub const LOGIN_URL: &str = "https://www.reddit.com/login";

//This can be done so much better
pub const USER_AGENTS: &str = r#"[
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/115.0",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36",
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36",
    "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/115.0",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/114.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.5.1 Safari/605.1.15",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:109.0) Gecko/20100101 Firefox/115.0",
    "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/114.0",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36 Edg/114.0.1823.67",
    "Mozilla/5.0 (Windows NT 10.0; rv:109.0) Gecko/20100101 Firefox/115.0",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36 Edg/114.0.1823.82",
    "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/115.0",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.5.2 Safari/605.1.15",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:109.0) Gecko/20100101 Firefox/114.0",
    "Mozilla/5.0 (X11; Linux x86_64; rv:102.0) Gecko/20100101 Firefox/102.0",
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36 Edg/115.0.1901.183",
    "Mozilla/5.0 (Windows NT 10.0; rv:114.0) Gecko/20100101 Firefox/114.0",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:102.0) Gecko/20100101 Firefox/102.0",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.0.0 Safari/537.36 OPR/99.0.0.0",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36 Edg/114.0.1823.58",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36 Edg/114.0.1823.79",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.5 Safari/605.1.15",
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/77.0.3865.75 Safari/537.36",
    "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/114.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36 OPR/100.0.0.0",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/116.0",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/111.0.0.0 Safari/537.36",
    "Mozilla/5.0 (X11; CrOS x86_64 14541.0.0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.3 Safari/605.1.15",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.4 Safari/605.1.15",
    "Mozilla/5.0 (Windows NT 10.0; rv:102.0) Gecko/20100101 Firefox/102.0",
    "Mozilla/5.0 (Windows NT 10.0; WOW64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.5666.197 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.2 Safari/605.1.15",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_2) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/79.0.3945.88 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.6.1 Safari/605.1.15",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/99.0.4844.51 Safari/537.36",
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36 Edg/114.0.1823.86",
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/113.0",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 YaBrowser/23.5.4.674 Yowser/2.5 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 YaBrowser/23.5.4.674 Yowser/2.5 Safari/537.36"
  ]"#;

impl RedditManager {
    pub fn get_random_ua() -> Result<String, Box<dyn std::error::Error>> {
        let user_agent_list: Vec<String> = serde_json::from_str(USER_AGENTS)?;

        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..user_agent_list.len());

        Ok(user_agent_list[random_index].clone())
    }

    pub fn initial_headers() -> Vec<(&'static str, String)> {
        let user_agent = match Self::get_random_ua() {
            Ok(user_agent) => user_agent,
            Err(_) => "".to_string(),
        };

        vec![
            ("origin", REDDIT_URL.to_string()),
            ("user-agent", user_agent),
        ]
    }

    pub async fn get_reddit_token(
        username: &String,
        password: &String,
    ) -> Result<Token, Box<dyn std::error::Error>> {
        let uname: String = username.clone();
        let passwd = password.clone();
        println!("Logging into reddit with {} and {}", uname, passwd);
        let mut headers = reqwest::header::HeaderMap::new();
        for (name, value) in Self::initial_headers() {
            headers.insert(
                HeaderName::from_bytes(name.as_bytes()).unwrap(),
                HeaderValue::from_str(&value).unwrap(),
            );
        }

        let client = ClientBuilder::new()
            .default_headers(headers.clone())
            .cookie_store(true)
            .build()?;

        let mut response = client.get(REDDIT_URL).send().await?;
        if !response.status().is_success() {
            log::error!(
                "Request to Reddit failed with status code: {}",
                response.status()
            );
            return Err(format!(
                "Request to Reddit failed with status code: {}",
                response.status()
            )
            .into());
        }

        // Get CSRF token from login page
        println!("Getting CSRF token...");
        let response_text_login = client.get(LOGIN_URL).send().await?.text().await?;

        let element = "<input type=\"hidden\" name=\"csrf_token\" value=\"";
        let start_index_found = response_text_login.find(element);

        let start_index = match start_index_found {
            Some(index) => index,
            None => 0,
        };

        let csrf_token: &String = &response_text_login[start_index + element.len()..]
            .chars()
            .take_while(|&char| char != '"')
            .collect::<String>();

        let form_data = [
            ("username", &uname.clone()),
            ("password", &passwd.clone()),
            ("dest", &REDDIT_URL.to_string().clone()),
            ("csrf_token", csrf_token),
        ];

        // Login
        response = client.post(LOGIN_URL).form(&form_data).send().await?;

        if !response.status().is_success() {
            let status = response.status().clone();
            log::error!("Login to Reddit failed with status code: {}", status);

            // Log response headers (if needed)
            for (name, value) in response.headers() {
                log::error!("Response Header: {}: {:?}", name, value);
            }

            // Log response body (if needed)
            let response_text = response.text().await?;
            log::error!("Response Body: {}", response_text);

            return Err(format!("Login to Reddit failed with status code: {}", status).into());
        }

        let mut reddit_session: Option<String> = None;

        for (header_name, header_value) in response.headers().iter() {
            if header_name == "set-cookie" {
                for cookie in header_value.to_str().unwrap().split(';') {
                    if cookie.trim().starts_with("reddit_session=") {
                        reddit_session = Some(
                            cookie
                                .trim_start_matches("reddit_session=")
                                .split('%')
                                .next()
                                .unwrap_or("")
                                .to_string(),
                        );
                        break; // Stop searching after finding the token
                    }
                }
            }
        }

        println!("{} {}", uname, "Login successful!".green());

        // Get new access token
        println!(
            "{} {} {}{}",
            uname,
            "Getting".green(),
            "reddit".red(),
            " access token...".green()
        );

        let reddit_response = client
            .get(REDDIT_URL)
            .headers(headers.clone())
            .send()
            .await?;

        let mut jwt_token: Option<String> = None;

        for (header_name, header_value) in reddit_response.headers().iter() {
            if header_name == "set-cookie" {
                for cookie in header_value.to_str().unwrap().split(';') {
                    if cookie.trim().starts_with("token_v2=") {
                        jwt_token = Some(cookie.trim_start_matches("token_v2=").to_string());
                        break; // Stop searching after finding the token
                    }
                }
            }
        }

        let token = match jwt_token {
            Some(token) => token,
            None => {
                println!("JWT Token not found in the 'token_v2' header.");
                "".into()
            }
        };

        let session = match reddit_session {
            Some(reddit_sesion) => reddit_sesion,
            None => {
                println!("reddit session not found in the 'reddit_session' header.");
                "".into()
            }
        };

        Ok(Token::new(session, token))
    }

    pub fn decode_jwt_and_get_expiry(mut jwt_token: &str) -> Result<i64, String> {
        jwt_token = jwt_token.trim_start_matches("Bearer ");

        match jwt_token.split('.').collect::<Vec<&str>>()[..] {
            [_, payload, _] => {
                let decoded_payload = Self::base64url_decode(payload)
                    .map_err(|e| format!("Failed to decode payload: {}", e))?;

                let payload_data: HashMap<String, Value> = serde_json::from_slice(&decoded_payload)
                    .map_err(|e| format!("Failed to parse JSON payload: {}", e))?;

                match payload_data.get("exp") {
                    Some(exp_value) => match exp_value.as_i64() {
                        Some(expiration_time) => Ok(expiration_time),
                        None => Err("Invalid 'exp' value in payload".to_string()),
                    },
                    None => Err("Token does not have an expiry date!".to_string()),
                }
            }
            _ => Err("Invalid JWT format".to_string()),
        }
    }

    pub async fn refresh_token_if_needed(token: Token) -> Result<Token, Box<dyn std::error::Error>> {
        let timestamp = match Self::decode_jwt_and_get_expiry(&token.jwt_token) {
            Ok(timestamp) => timestamp,
            Err(_) => 0,
        };
        if !Self::is_expired(timestamp as f64) {
            let mut headers = reqwest::header::HeaderMap::new();
            for (name, value) in Self::initial_headers() {
                headers.insert(
                    HeaderName::from_bytes(name.as_bytes()).unwrap(),
                    HeaderValue::from_str(&value).unwrap(),
                );
            }

            headers.insert(
                "set-cookie",
                HeaderValue::from_str(&format!("reddit_session={}", token.reddit_session)).unwrap(),
            );

            let client = ClientBuilder::new()
                .default_headers(headers.clone())
                .cookie_store(true)
                .build()?;

            let reddit_response = client
                .get(REDDIT_URL)
                .headers(headers.clone())
                .send()
                .await?;

                let mut jwt_token: Option<String> = None;

                for (header_name, header_value) in reddit_response.headers().iter() {
                    if header_name == "set-cookie" {
                        for cookie in header_value.to_str().unwrap().split(';') {
                            if cookie.trim().starts_with("token_v2=") {
                                jwt_token = Some(cookie.trim_start_matches("token_v2=").to_string());
                                break; // Stop searching after finding the token
                            }
                        }
                    }
                }

                let jwt_token = match jwt_token {
                    Some(token) => token,
                    None => {
                        println!("JWT Token not found in the 'token_v2' header.");
                        "".into()
                    }
                };
        
            return Ok(Token::new(token.reddit_session, jwt_token));
        }

        Ok(token)
    }

    pub fn is_expired(auth_token_expires_at: f64) -> bool {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards!");
        let expires_at_duration = Duration::from_secs_f64(auth_token_expires_at);
        current_time > expires_at_duration
    }

    pub fn base64url_decode(input_str: &str) -> Result<Vec<u8>, base64::DecodeError> {
        let decoder = engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);
        let decoded = decoder.decode(input_str)?;
        Ok(decoded)
    }
}
