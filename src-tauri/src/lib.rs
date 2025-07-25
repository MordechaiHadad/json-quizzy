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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let migrations = vec![Migration {
        version: 1,
        description: "create relational quiz tables",
        sql: V1_CREATE_QUIZ_TABLES,
        kind: MigrationKind::Up,
    }];

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:jsonquizzy.db", migrations)
                .build(),
        )
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
