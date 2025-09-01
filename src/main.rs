mod cli;
mod generator;
mod utils;

use clap::Parser;
use cli::{Cli, Commands, GenerateCommands};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { name } => {
            if std::path::Path::new(name).exists() {
                eprintln!("Erreur: Le dossier '{}' existe déjà.", name);
                std::process::exit(1);
            }
            generator::generate_new_project(name);
        }
        Commands::Generate { command } => {
            match command {
                GenerateCommands::Controller { name } => {
                    generator::generate_controller(name);
                }
            }
        }
    }
}