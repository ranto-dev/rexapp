use inquire::Select;

pub struct ProjectChoices {
    pub db_choice: String,
    pub cors_choice: String,
    pub project_type: String,
}

pub fn ask_for_choices() -> ProjectChoices {
    let db_options: Vec<&'static str> = vec!["PostgreSQL", "MongoDB", "Aucune"];
    let db: String = Select::new("Quelle base de données voulez-vous utiliser ?", db_options)
        .prompt()
        .unwrap()
        .to_string();

    let cors_options = vec!["Oui", "Non"];
    let cors = Select::new("Voulez-vous installer CORS ?", cors_options)
        .prompt()
        .unwrap()
        .to_string();

    let project_type_options = vec!["API (sans template HTML)", "API avec template"];
    let project_type = Select::new("Quel type de projet voulez-vous créer ?", project_type_options)
        .prompt()
        .unwrap()
        .to_string();

    ProjectChoices {
        db_choice: db,
        cors_choice: cors,
        project_type,
    }
}