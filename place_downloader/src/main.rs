use menus::main_menu::MainMenu;

mod menus;
mod models;
mod managers;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    MainMenu::main_menu().await;
    Ok(())
}
