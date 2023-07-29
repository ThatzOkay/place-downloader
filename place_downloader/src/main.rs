use menus::main_menu::MainMenu;
use env_logger;

mod menus;
mod models;
mod managers;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    MainMenu::main_menu().await;
    Ok(())
}
