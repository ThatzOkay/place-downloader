use reqwest::{header::{HeaderMap, HeaderValue}, Client};
use serde_json::Value;

pub struct RedditManager;

//TODO move to config
pub const REDDIT_URL: &str = "https://www.reddit.com";
pub const LOGIN_URL: &str = "https://www.reddit.com/login";

impl RedditManager {
    pub fn initial_headers() -> HeaderMap {
        let mut headers = HeaderMap::new();

        headers.insert("accept", HeaderValue::from_static("*/*"));
        headers.insert(
            "accept-encoding",
            HeaderValue::from_static("gzip, deflate, br"),
        );
        headers.insert("accept-language", HeaderValue::from_static("en"));
        headers.insert(
            "content-type",
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        );
        // Note: REDDIT_URL should be a valid URL string, otherwise, this will cause an error.
        headers.insert("origin", HeaderValue::from_static(REDDIT_URL));
        headers.insert("sec-fetch-mode", HeaderValue::from_static("cors"));
        headers.insert("sec-fetch-site", HeaderValue::from_static("same-origin"));
        headers.insert("user-agent", HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36"));

        headers
    }

    pub async fn get_reddit_token(
        username: &String,
        password: &String,
    ) -> Result<String, Box<dyn std::error::Error>> {
        println!("Logging into reddit with {}", username);
        let client = Client::new();
        let headers = Self::initial_headers();

        let response = client.get(REDDIT_URL).headers(headers.clone()).send().await?;
        if !response.status().is_success() {
            return Err(format!(
                "Request to Reddit failed with status code: {}",
                response.status()
            )
            .into());
        }

        // Get CSRF token from login page
        println!("Getting CSRF token...");
        let mut response_text = client.get(LOGIN_URL).headers(headers.clone()).send().await?.text().await?;
        let csrf_token = {
            let document = scraper::Html::parse_document(&response_text);
            let csrf_token = document
                .select(&scraper::Selector::parse("input[name=csrf_token]").unwrap())
                .next()
                .and_then(|input| input.value().attr("value"))
                .ok_or("Could not find CSRF token")?;
            csrf_token.to_string()
        };

        // Login
        response_text = client
            .post(LOGIN_URL).headers(headers)
            .form(&[
                ("username", username),
                ("password", password),
                ("dest", &REDDIT_URL.to_string()),
                ("csrf_token", &csrf_token),
            ])
            .send()
            .await?
            .text()
            .await?;

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
