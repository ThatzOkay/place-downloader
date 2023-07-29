
pub struct Token {
    pub reddit_session: String,
    pub jwt_token: String
}

impl Token {
    pub fn new(reddit_session: String, jwt_token: String) -> Self {
        Token {
            reddit_session,
            jwt_token
        }
    }
}