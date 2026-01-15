mod cli;
mod todo;
mod storage;
mod error;

use cli::Cli;
use clap::Parser;
use storage::TodoStorage;

fn main() {
    let cli = Cli::parse();
    let mut storage = TodoStorage::load("todos.json").unwrap_or_default();

    match cli.command.execute(&mut storage) {
        Ok(_) => {
            storage.save("todos.json").expect("Failed to save todos");
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
