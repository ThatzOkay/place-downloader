
pub struct AccountToken {
    username: String,
    jwt_token: String
}

impl AccountToken {
    pub fn new(username: Stirng, jwt_token: String) -> Self{
        AccountToken {
            username,
            jwt_token
        }
    }
}