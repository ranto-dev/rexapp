use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(author="ranto-dev", version, about = "Générateur de projet API REST Node.js", long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Crée un nouveau projet d'API REST 
    New {
        /// Le nom du projet
        name: String,
    },
}
fn generate_project(name: &str) {
    let project_dir = Path::new(name);

    if project_dir.exists() {
        eprintln!("Erreur: Le dossier '{}' existe déjà.", name);
        std::process::exit(1);
    }

    // Création de la structure de dossiers de base
    if let Err(e) = fs::create_dir_all(project_dir.join("src/routes")) {
        eprintln!("Erreur lors de la création du dossier: {}", e);
        return;
    }
    if let Err(e) = fs::create_dir_all(project_dir.join("src/controllers")) {
        eprintln!("Erreur lors de la création du dossier: {}", e);
        return;
    }
    if let Err(e) = fs::create_dir_all(project_dir.join("src/models")) {
        eprintln!("Erreur lors de la création du dossier: {}", e);
        return;
    }

    // Création des fichiers de base (index.js, package.json, etc.)
    let package_json_content = format!(
        r#"{{
  "name": "{}",
  "version": "1.0.0",
  "description": "",
  "main": "src/index.js",
  "scripts": {{
    "start": "node src/index.js"
  }},
  "dependencies": {{
    "express": "^4.17.1"
  }}
}}"#,
        name
    );
    if let Err(e) = fs::write(project_dir.join("package.json"), package_json_content) {
        eprintln!("Erreur lors de la création de package.json: {}", e);
        return;
    }

    let index_js_content = r#"
const express = require('express');
const app = express();
const port = 3000;

app.get('/', (req, res) => {
  res.send('Hello World!');
});

app.listen(port, () => {
  console.log(`Serveur démarré sur http://localhost:${port}`);
});
"#;
    if let Err(e) = fs::write(project_dir.join("src/index.js"), index_js_content) {
        eprintln!("Erreur lors de la création de src/index.js: {}", e);
        return;
    }

    println!("Projet '{}' généré avec succès ! 😊", name);
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { name } => {
            println!("Génération du projet {}...", name);
            generate_project(name);
        }
    }
}