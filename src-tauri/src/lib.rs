use std::path::Path;

use dotenvy::from_path;
use tauri_plugin_sql::{Migration, MigrationKind};

const V1_CREATE_QUIZ_TABLES: &'static str = r#"
CREATE TABLE quizzes (
    id                  INTEGER PRIMARY KEY AUTOINCREMENT,
    title               TEXT NOT NULL,
    questions_json      TEXT NOT NULL,
    success_percentage  REAL,
    created_at          TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

"#;

#[tauri::command]

fn get_env_var(name: &str) -> String {
    let env_var = std::env::var(name).expect(&format!("{name} must be set."));

    env_var
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let migrations = vec![Migration {
        version: 1,
        description: "create relational quiz tables",
        sql: V1_CREATE_QUIZ_TABLES,
        kind: MigrationKind::Up,
    }];

    tauri::Builder::default()
        .setup(|_app| {
            let env_path = Path::new("../../json-quizzy.env");

            match from_path(env_path) {
                Ok(_) => {
                    println!(
                        "Successfully loaded environment variables from: {:?}",
                        env_path
                    );
                }

                Err(e) => {
                    eprintln!("Could not load .env file from {:?}: {}", env_path, e);

                    // You might want to panic here if the config is essential

                    std::process::exit(1);
                }
            }

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:jsonquizzy.db", migrations)
                .build(),
        )
        .invoke_handler(tauri::generate_handler![get_env_var])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
