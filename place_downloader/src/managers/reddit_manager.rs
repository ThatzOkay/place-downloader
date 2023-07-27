use reqwest::Client;

pub struct RedditManager;

impl RedditManager {

    //TODO move to config
    const REDDIT_URL: &String = "https://www.reddit.com";
    const LOGIN_URL: &String = "https://www.reddit.com/login";

    pub fn get_reddit_token(username: &String, password: &String) -> Result<String, Box<dyn std::error::Error>> {
        println!("Logging into reddit with {}", username);
        let client = Client::new();
        let mut response = client.get(REDDIT_URL).send()?.text()?;

        // Get CSRF token from login page
        println!("Getting CSRF token...");
        let csrf_token = {
            let document = scraper::Html::parse_document(&s);
            let csrf_token = document
                .select(&scraper::Selector::parse("input[name=csrf_token]").unwrap())
                .next()
                .and_then(|input| input.value().attr("value"))
                .ok_or("Could not find CSRF token")?;
            csrf_token.to_string()
        };

        // Login
        response = client.post(LOGIN_URL)
        .form(&[
            ("username", username),
            ("password", password),
            ("dest", REDDIT_URL),
            ("csrf_token", &csrf_token),
        ])
        .send()?
        .text()?;
    
        let data_str = {
            let document = scraper::Html::parse_document(&s);
            let script = document
                .select(&scraper::Selector::parse("script#data").unwrap())
                .next()
                .ok_or("Could not find data script")?;
            let content = script.inner_html();
            content.trim_start_matches("window.__r = ").trim_end_matches(';')
        };
    
        let data = serde_json::from_str(data_str)?;
    
        let token = format!("Bearer {}", data["user"]["session"]["accessToken"].as_str().ok_or("No access token")?);
        Ok(token)
    }
}