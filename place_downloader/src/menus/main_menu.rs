use std::io;
use std::process::exit;
use async_recursion::async_recursion;

use crate::menus::account_menu::AccountMenu;
use crate::menus::download_menu::DownloadMenu;

pub struct MainMenu;

impl MainMenu {
    
    #[async_recursion]
    pub async fn main_menu() {
        let mut selected_option = String::new();

        println!("Welcome to the r/place downloader.");
        println!("What would you like to do.");
        println!("1. Account managment");
        println!("2. Start downloader.");
        println!("3. Exit.");


        match io::stdin().read_line(&mut selected_option) {
            Ok(_) => {
                Self::parse_selected_option_main_menu(&mut selected_option).await;
            }
            Err(error) => {
                eprintln!("Error reading input: {}", error);
            }
        }
    }

    pub async fn parse_selected_option_main_menu(selected_option: &String) {
        match selected_option.trim().parse::<u32>() {
            Ok(option_num) => match option_num {
                1 => AccountMenu::account_management().await,
                2 => DownloadMenu::start_downloader().await,
                3 => exit(0),
                _ => {
                    println!("Invalid option. Please select a valid option.");
                    self::MainMenu::main_menu().await;
                }
            },
            Err(_) => {
                println!("Invalid input. Please enter a number corresponding to the option.");
                self::MainMenu::main_menu().await;
            }
        }
    }
}
