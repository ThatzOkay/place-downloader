use colored::Colorize;
use log;
use reqwest::{
    cookie::Jar,
    header::{HeaderName, HeaderValue},
    Client, ClientBuilder, Url,
};
use serde_json::Value;
use std::{fs, sync::Arc};

pub struct RedditManager;

// TODO: Move to config
pub const REDDIT_URL: &str = "https://www.reddit.com/";
pub const LOGIN_URL: &str = "https://www.reddit.com/login";

impl RedditManager {
    pub fn initial_headers() -> &'static [(&'static str, &'static str)] {
        &[
            ("accept", "*/*"),
            ("accept-encoding", "gzip, deflate, br"),
            ("accept-language", "en"),
            ("content-type", "application/x-www-form-urlencoded"),
            ("origin", REDDIT_URL),
            // # these headers seem to break the login
            // ("sec-ch-ua", r#""Not.A/Brand";v="8", "Chromium";v="114", "Google Chrome";v="114""#),
            // ("sec-ch-ua-mobile", "?0"),
            // ("sec-ch-ua-platform", r#""Windows""#),
            // ("sec-fetch-dest", "empty"),
            ("sec-fetch-mode", "cors"),
            ("sec-fetch-site", "same-origin"),
            ("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36"),
        ]
    }

    pub async fn get_reddit_token(
        username: &String,
        password: &String,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let uname: String = username.clone();
        let passwd = password.clone();
        println!("Logging into reddit with {} and {}", uname, passwd);
        let mut headers = reqwest::header::HeaderMap::new();
        for (name, value) in Self::initial_headers() {
            headers.insert(
                HeaderName::from_bytes(name.as_bytes()).unwrap(),
                HeaderValue::from_str(value).unwrap(),
            );
        }

        let jar = Arc::new(Jar::default());

        let client = ClientBuilder::new()
            .default_headers(headers.clone())
            .cookie_provider(Arc::clone(&jar))
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

        let csrf_token: &String = &response_text_login[start_index + element.len()..].chars().take_while(|&char| char!='"' ).collect::<String>();

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
            Some(token) => {
                println!("JWT Token: {}", token);
                token
            }
            None => {
                println!("JWT Token not found in the 'token_v2' header.");
                "".into()
            }
        };

        Ok(token)
    }
}
