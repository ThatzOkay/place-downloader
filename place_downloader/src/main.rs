mod models;
mod managers;

use std::io;

use managers::{config_manager::ConfigManager, reddit_manager::RedditManager};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let token = RedditManager.get_reddit_token("PoahDasMienMerkBier", "b%JU@&7no2ew7T");
    println!(token);
    main_menu();

    Ok(())
}


fn main_menu() {
    let mut selected_option = String::new();

    println!("Welcome to the r/place downloader.");
    println!("What would you like to do.");
    println!("1. Account managment");
    println!("2. Start downloader.");

    match io::stdin().read_line(&mut selected_option) {
        Ok(_) => {
            parse_selected_option_main_menu(&mut selected_option);
        }
        Err(error) => {
            eprintln!("Error reading input: {}", error);
        }
    }
}

fn account_management() {
    println!("1. Add account");
    println!("2. Edit account");
    println!("3. Remove account");
}

fn start_downloader() {
    let config = match ConfigManager::load_config() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Error loading configuration: {}", err);
            return;
        }
    };

    if config.accounts.len() == 0 {
        println!("No accounts in config. Back to main menu");
        main_menu();
    }


}

fn parse_selected_option_main_menu(selected_option: &String) {
    match selected_option.trim().parse::<u32>() {
        Ok(option_num) => {
            match option_num {
                1 => account_management(),
                2 => start_downloader(),
                _ => {
                    println!("Invalid option. Please select a valid option.");
                    main_menu()
                },
            }
        }
        Err(_) => {
            println!("Invalid input. Please enter a number corresponding to the option.");
        }
    }
}

fn parse_selected_option_account_menu(selected_option: &String) {

}
