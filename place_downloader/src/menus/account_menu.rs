use crate::managers::config_manager::ConfigManager;
use crate::models::config_model::{Account, AppConfig};
use async_recursion::async_recursion;
use std::io;

use super::main_menu::MainMenu;

pub struct AccountMenu;

impl AccountMenu {
    
    #[async_recursion]
    pub async fn account_management() {
        let mut selected_option = String::new();

        println!("1. Add account");
        println!("2. Edit account");
        println!("3. Remove account");
        println!("4. Back");

        match io::stdin().read_line(&mut selected_option) {
            Ok(_) => {
                Self::parse_selected_option_account_menu(&mut selected_option).await;
            }
            Err(error) => {
                eprintln!("Error reading input: {}", error);
            }
        }
    }

    pub async fn parse_selected_option_account_menu(selected_option: &String) {
        match selected_option.trim().parse::<u32>() {
            Ok(option_num) => match option_num {
                1 => Self::add_account().await,
                2 => Self::edit_account(),
                3 => Self::remove_account().await,
                4 => MainMenu::main_menu().await,
                _ => {
                    println!("Invalid option. Please select a valid option.");
                    Self::account_management().await;
                }
            },
            Err(_) => {
                println!("Invalid input. Please enter a number corresponding to the option.");
                Self::account_management().await;
            }
        }
    }

    
    #[async_recursion]
    pub async fn add_account() {
        let mut selected_option = String::new();

        println!("1. Add account using username and password");
        println!("2. Add account using JWT token");
        println!("3. Back");

        match io::stdin().read_line(&mut selected_option) {
            Ok(_) => {
                Self::parse_selected_option_add_account(&mut selected_option).await;
            }
            Err(error) => {
                eprintln!("Error reading input: {}", error);
            }
        }
    }

    pub async fn parse_selected_option_add_account(selected_option: &String) {
        match selected_option.trim().parse::<u32>() {
            Ok(option_num) => match option_num {
                1 => Self::add_normal_account().await,
                2 => Self::add_jwt_account().await,
                3 => Self::account_management().await,
                _ => {
                    println!("Invalid option. Please select a valid option.");
                    Self::account_management().await;
                }
            },
            Err(_) => {
                println!("Invalid input. Please enter a number corresponding to the option.");
                Self::account_management().await;
            }
        }
    }

    pub async fn add_normal_account() {
        let mut new_username = String::new();
        let mut new_password = String::new();

        println!("Input username");
        match io::stdin().read_line(&mut new_username) {
            Ok(_) => {}
            Err(error) => {
                eprintln!("Error reading input: {}", error);
                Self::account_management().await;
            }
        }

        let exists = Self::does_account_exist(new_username.clone());
        if exists.is_some() {
            println!("Account already exists");
            Self::account_management().await;
        }

        println!("Input password");
        match io::stdin().read_line(&mut new_password) {
            Ok(_) => {}
            Err(error) => {
                eprintln!("Error reading input: {}", error);
                Self::account_management().await;
            }
        }

        let account = Account::with_credentials(new_username, new_password);

        match ConfigManager::load_config() {
            Ok(app_config) => {
                let mut accounts = app_config.accounts.clone();

                // Wrap the account in a Vec and append it to the existing accounts
                accounts.push(account);

                let config = AppConfig::new(accounts);
                match ConfigManager::save_config(&config) {
                    Ok(()) => println!("Configuration saved successfully."),
                    Err(error) => eprintln!("Error saving configuration: {}", error),
                }
                Self::account_management().await;
            }
            Err(error) => {
                eprintln!("Error loading configuration: {}", error);
                Self::account_management().await;
            }
        }
    }

    pub async fn add_jwt_account() {
        let mut new_username = String::new();
        let mut new_jwt_token = String::new();

        println!("Input username");
        match io::stdin().read_line(&mut new_username) {
            Ok(_) => {}
            Err(error) => {
                eprintln!("Error reading input: {}", error);
                Self::account_management().await;
            }
        }

        let exists = Self::does_account_exist(new_username.clone());
        if exists.is_some() {
            println!("Account already exists");
            Self::account_management().await;
        }

        println!("Input JWT");
        match io::stdin().read_line(&mut new_jwt_token) {
            Ok(_) => {}
            Err(error) => {
                eprintln!("Error reading input: {}", error);
                Self::account_management().await;
            }
        }

        let account: Account = Account::with_token(new_username, new_jwt_token);

        match ConfigManager::load_config() {
            Ok(app_config) => {
                let mut accounts = app_config.accounts.clone();

                // Wrap the account in a Vec and append it to the existing accounts
                accounts.push(account);

                let config = AppConfig::new(accounts);
                match ConfigManager::save_config(&config) {
                    Ok(()) => println!("Configuration saved successfully."),
                    Err(error) => eprintln!("Error saving configuration: {}", error),
                }
                Self::account_management().await;
            }
            Err(error) => {
                eprintln!("Error loading configuration: {}", error);
                Self::account_management().await;
            }
        }
    }

    pub fn edit_account() {}

    #[async_recursion]
    pub async fn remove_account() {
        match ConfigManager::load_config() {
            Ok(app_config) => {
                let mut accounts = app_config.accounts.clone();

                if accounts.len() == 0 {
                    println!("No accounts exist in config.");
                    Self::account_management().await;
                }

                println!("Select account to remove or go back");

                for (index, account) in accounts.iter().enumerate() {
                    println!("{}. Remove {}", index + 1, account.username)
                }

                println!("{}. Back", accounts.len() + 1);

                let mut selected_option = String::new();
                match io::stdin().read_line(&mut selected_option) {
                    Ok(_) => {}
                    Err(error) => {
                        eprintln!("Error reading input: {}", error);
                        Self::remove_account().await;
                    }
                }
                let back_number = accounts.len() + 1;
                match selected_option.trim().parse::<usize>() {
                    Ok(option_num) => match option_num {
                        num if num == back_number => {
                            Self::account_management().await;
                        }
                        _ => {
                            match accounts.get(option_num - 1) {
                                Some(_) => {
                                    accounts.remove(option_num - 1);

                                    let config = AppConfig::new(accounts);
                                    match ConfigManager::save_config(&config) {
                                        Ok(()) => println!("Configuration saved successfully."),
                                        Err(error) => {
                                            eprintln!("Error saving configuration: {}", error)
                                        }
                                    }
                                    Self::account_management().await;
                                }
                                None => {
                                    // Handle the case when the index is out of bounds
                                    println!(
                                        "Invalid option. The selected index is out of bounds."
                                    );
                                    Self::account_management().await;
                                }
                            }
                        }
                    },
                    Err(_) => {
                        println!(
                            "Invalid input. Please enter a number corresponding to the option."
                        );
                        Self::account_management().await;
                    }
                }
            }
            Err(error) => {
                eprintln!("Error loading configuration: {}", error);
                Self::account_management().await;
            }
        }
    }

    pub fn does_account_exist(username: String) -> Option<Account> {
        match ConfigManager::load_config() {
            Ok(app_config) => {
                let accounts = app_config.accounts.clone();

                accounts
                    .iter()
                    .find(|acc| acc.username == username)
                    .cloned()
            }
            Err(error) => {
                eprintln!("Error loading configuration: {}", error);
                None
            }
        }
    }
}
