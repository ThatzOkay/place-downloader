use reqwest::{
    cookie::Jar,
    header::{HeaderName, HeaderValue},
    ClientBuilder,
};
use serde_json::Value;
use std::sync::Arc;

pub struct RedditManager;

// TODO: Move to config
pub const REDDIT_URL: &str = "https://www.reddit.com";
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
            .default_headers(headers)
            .cookie_provider(Arc::clone(&jar))
            .cookie_store(true)
            .build()?;

        let mut response = client.get(REDDIT_URL).send().await?;
        if !response.status().is_success() {
            return Err(format!(
                "Request to Reddit failed with status code: {}",
                response.status()
            )
            .into());
        }

        // Get CSRF token from login page
        println!("Getting CSRF token...");
        let mut response_text = client.get(LOGIN_URL).send().await?.text().await?;
        let csrf_token = {
            let document = scraper::Html::parse_document(&response_text);
            let csrf_token = document
                .select(&scraper::Selector::parse("input[name=csrf_token]").unwrap())
                .next()
                .and_then(|input| input.value().attr("value"))
                .ok_or("Could not find CSRF token")?;
            csrf_token.to_string()
        };

        let form_data = [
            ("username", &uname.clone()),
            ("password", &passwd.clone()),
            ("dest", &REDDIT_URL.to_string().clone()),
            ("csrf_token", &csrf_token.clone()),
        ];

        // Login
        response = client
            .post(LOGIN_URL)
            .form(&form_data)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!(
                "Login to Reddit failed with status code: {}",
                response.status()
            )
            .into());
        }

        response_text = response.text().await?;

        let data_str = {
            let document = scraper::Html::parse_document(&response_text);
            println!("{}", response_text);
            let script = document
                .select(&scraper::Selector::parse("script#data").unwrap())
                .next()
                .ok_or("Could not find data script")?;
            let content = script.inner_html();
            let content_owned = content
                .trim_start_matches("window.__r = ")
                .trim_end_matches(';')
                .to_string();
            content_owned
        };

        let data: Value = serde_json::from_str(&data_str)?;

        let token = format!(
            "Bearer {}",
            data["user"]["session"]["accessToken"]
                .as_str()
                .ok_or("No access token")?
        );
        Ok(token)
    }
}
