use clap::{Parser, Subcommand};
use crate::storage::TodoStorage;
use crate::error::AppResult;

#[derive(Parser)]
#[command(name = "rusty-todo")]
#[command(about = "A simple Rust CLI Todo app")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add { title: String },
    List,
    Done { id: usize },
    Remove { id: usize },
}

impl Commands {
    pub fn execute(&self, storage: &mut TodoStorage) -> AppResult<()> {
        match self {
            Commands::Add { title } => {
                storage.add(title);
                println!("Todo added");
            }
            Commands::List => {
                storage.list();
            }
            Commands::Done { id } => {
                storage.mark_done(*id)?;
                println!("Todo marked as done");
            }
            Commands::Remove { id } => {
                storage.remove(*id)?;
                println!("Todo removed");
            }
        }
        Ok(())
    }
}
