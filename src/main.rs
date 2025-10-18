mod database;
mod errors;
mod models;
mod utils;

use database::manager::CommandManager;
use std::process;
use utils::cli::handle_cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut command_manager = match CommandManager::new() {
        Ok(manager) => manager,
        Err(e) => {
            eprintln!("Failed to initialize command manager: {}", e);
            process::exit(1);
        }
    };

    if let Err(e) = handle_cli(&mut command_manager) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }

    Ok(())
}
