use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about = "Générateur de projet API REST Node.js", long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Crée un nouveau projet d'API REST
    New {
        /// Le nom du projet
        name: String,
    },

    /// Crée un nouveau contrôleur pour une entité
    Generate {
        #[command(subcommand)]
        command: GenerateCommands,
    }
}

#[derive(Subcommand)]
pub enum GenerateCommands {
    /// Crée un nouveau contrôleur pour une entité
    Controller {
        /// Le nom de l'entité (ex: User)
        name: String,
    },
}