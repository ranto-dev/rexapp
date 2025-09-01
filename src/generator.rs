use crate::utils::questions::{ask_for_choices, ProjectChoices};
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::Duration;

pub fn generate_new_project(name: &str) {
    println!("Génération du projet '{}'...", name);

    let choices = ask_for_choices();


    let project_dir = Path::new(name);
    let pb_dirs = ProgressBar::new_spinner();
    pb_dirs.set_style(ProgressStyle::with_template("{spinner:.green} {msg}").unwrap());
    pb_dirs.set_message("Création des dossiers du projet...");
    pb_dirs.enable_steady_tick(Duration::from_millis(100));

    let dirs_to_create = vec![
        project_dir.join("src/routes"),
        project_dir.join("src/controllers"),
        project_dir.join("src/models"),
    ];

    for dir in dirs_to_create {
        if let Err(e) = fs::create_dir_all(&dir) {
            eprintln!("Erreur lors de la création du dossier {:?}: {}", dir, e);
            pb_dirs.finish_with_message("Échec de la création des dossiers.");
            return;
        }
    }

    if choices.project_type == "API avec template" {
        if let Err(e) = fs::create_dir_all(project_dir.join("public")) {
            eprintln!("Erreur lors de la création du dossier 'public': {}", e);
            pb_dirs.finish_with_message("Échec de la création des dossiers.");
            return;
        }
    }

    pb_dirs.finish_with_message("Structure de dossiers créée. ✅");

    let mut dependencies = vec![("express", "^4.17.1")];
    if choices.db_choice == "PostgreSQL" {
        dependencies.push(("pg", "^8.0.0"));
    } else if choices.db_choice == "MongoDB" {
        dependencies.push(("mongoose", "^6.0.0"));
    }
    if choices.cors_choice == "Oui" {
        dependencies.push(("cors", "^2.8.5"));
    }
    dependencies.push(("dotenv", "^16.0.0"));


    let dev_dependencies = vec![("nodemon", "^2.0.0")];

    let dependencies_str: String = dependencies.iter()
        .map(|(dep, ver)| format!("    \"{}\": \"{}\"", dep, ver))
        .collect::<Vec<_>>()
        .join(",\n");

    let dev_dependencies_str: String = dev_dependencies.iter()
        .map(|(dep, ver)| format!("    \"{}\": \"{}\"", dep, ver))
        .collect::<Vec<_>>()
        .join(",\n");

    let package_json_content = format!(
        r#"{{
  "name": "{}",
  "version": "1.0.0",
  "description": "",
  "main": "src/index.js",
  "scripts": {{
    "start": "node src/index.js",
    "dev": "nodemon src/index.js"
  }},
  "dependencies": {{
{}
  }},
  "devDependencies": {{
{}
  }}
}}"#,
        name,
        dependencies_str,
        dev_dependencies_str
    );
    fs::write(project_dir.join("package.json"), package_json_content)
        .expect("Échec de l'écriture de package.json");

    // 5. Créer le fichier `index.js`, `.env`, etc.
    let mut index_js_content = String::new();
    index_js_content.push_str("require('dotenv').config();\n");
    index_js_content.push_str("const express = require('express');\n");
    if choices.cors_choice == "Oui" {
        index_js_content.push_str("const cors = require('cors');\n");
    }
    index_js_content.push_str("const app = express();\nconst port = process.env.PORT || 3000;\n\n");
    index_js_content.push_str("app.use(express.json());\n");

    if choices.cors_choice == "Oui" {
        index_js_content.push_str("app.use(cors());\n");
    }

    if choices.project_type == "API avec template" {
        index_js_content.push_str("app.use(express.static('public'));\n");
        let html_content = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Mon API</title>
    <style>
        body { font-family: sans-serif; display: flex; justify-content: center; align-items: center; height: 100vh; margin: 0; background-color: #f0f0f0; }
        h1 { color: #333; }
        p { color: #666; }
    </style>
</head>
<body>
    <div>
        <h1>Bienvenue sur mon API !</h1>
        <p>Le serveur fonctionne. 😊</p>
    </div>
</body>
</html>
"#;
        fs::write(project_dir.join("public/index.html"), html_content)
            .expect("Échec de l'écriture du template HTML");
    }

    index_js_content.push_str("\napp.get('/', (req, res) => {\n  res.send('Hello World!');\n});\n\n");

    if choices.db_choice == "PostgreSQL" {
        index_js_content.push_str("// Logique de connexion à PostgreSQL...\n");
    } else if choices.db_choice == "MongoDB" {
        index_js_content.push_str("// Logique de connexion à MongoDB...\n");
    }

    index_js_content.push_str(&format!(
        "app.listen(port, () => {{\n  console.log(`Serveur démarré sur http://localhost:${{port}}`);\n}});\n"
    ));

    fs::write(project_dir.join("src/index.js"), index_js_content)
        .expect("Échec de l'écriture de index.js");

    fs::write(project_dir.join(".env"), "PORT=3000\n").expect("Échec de l'écriture de .env");

    let pb_install = ProgressBar::new_spinner();
    pb_install.set_style(ProgressStyle::with_template("{spinner:.cyan} {msg}").unwrap());
    pb_install.set_message("Installation des dépendances...");
    pb_install.enable_steady_tick(Duration::from_millis(100));

    let output = Command::new("npm")
        .arg("install")
        .current_dir(project_dir)
        .output();
    
    pb_install.finish_with_message("Dépendances installées ! ✅");

    match output {
        Ok(output) => {
            if output.status.success() {
                println!("\nProjet '{}' généré avec succès ! 🎉", name);
                println!("Pour démarrer le projet :");
                println!("  cd {}", name);
                println!("  npm start");
                println!("Ou en mode développement (rechargement automatique) :");
                println!("  npm run dev");
            } else {
                eprintln!("\nErreur lors de l'installation des dépendances.");
                eprintln!("Sortie d'erreur de npm:\n{}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => {
            eprintln!("\nErreur lors de l'exécution de npm: {}", e);
        }
    }
}