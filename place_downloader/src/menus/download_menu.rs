pub struct DownloadMenu;

use crate::managers::reddit_manager::RedditManager;
use crate::menus::main_menu::MainMenu;
use crate::managers::config_manager::ConfigManager;

impl DownloadMenu {
    pub async fn start_downloader() {
        let config = match ConfigManager::load_config() {
            Ok(config) => config,
            Err(err) => {
                eprintln!("Error loading configuration: {}", err);
                return;
            }
        };

        if config.accounts.len() == 0 {
            println!("No accounts in config. Back to main menu");
            MainMenu::main_menu().await;
        }

        match ConfigManager::load_config() {
            Ok(app_config) => {
                let accounts = app_config.accounts.clone();
                
                for account in accounts {
                    if account.jwt_token.is_none() && account.password.is_some() {
                        let password = account.password.unwrap();
                        
                        println!("Loggin in with {} and {}", account.username, password);
                        // Get token from reddit
                        match RedditManager::get_reddit_token(&account.username, &password).await {
                            Ok(token) => {
                                println!("session: {:#?}", token.reddit_session);
                                println!("jwt: {:#?}", token.jwt_token);
                                println!("timestamp: {:#?}", RedditManager::decode_jwt_and_get_expiry(&token.jwt_token));
                            },
                            Err(err) => eprintln!("Error: {}", err),
                        }
                    }
                }

            }
            Err(error) => {
                eprintln!("Error loading configuration: {}", error);
                MainMenu::main_menu().await;
            }
        }
    }
}
