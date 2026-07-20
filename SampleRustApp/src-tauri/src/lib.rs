mod commands;
mod models;
mod repositories;

use std::fs;

use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use tauri::Manager;

use commands::note_command::{create_note, delete_note, get_note, get_notes, update_note};
use repositories::note_repository::NoteRepository;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
     tauri::Builder::default()
        .setup(|app| {
            /*
             * OSが管理するアプリ専用データディレクトリを取得する。
             *
             * Windowsでは、おおむね次の場所になる:
             * C:\Users\<ユーザー名>\AppData\Roaming\<identifier>
             */
            let app_data_dir = app.path().app_data_dir()?;

            fs::create_dir_all(&app_data_dir)?;

            let database_path = app_data_dir.join("sample.db");

            let connection_options = SqliteConnectOptions::new()
                .filename(database_path)
                .create_if_missing(true)
                .foreign_keys(true);

            /*
             * setup自体は同期クロージャなので、
             * Tauriの非同期ランタイム上でDB初期化を完了させる。
             */
            let pool = tauri::async_runtime::block_on(async {
                let pool = SqlitePoolOptions::new()
                    .max_connections(5)
                    .connect_with(connection_options)
                    .await?;

                sqlx::migrate!("./migrations")
                    .run(&pool)
                    .await?;

                Ok::<_, Box<dyn std::error::Error>>(pool)
            })?;

            let repository = NoteRepository::new(pool);

            /*
             * Repositoryをアプリ全体の共有状態として登録する。
             * CommandからState<NoteRepository>として取得できる。
             */
            app.manage(repository);

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            commands::note_command::create_note,
            commands::note_command::get_notes,
            commands::note_command::get_note,
            commands::note_command::update_note,
            commands::note_command::delete_note,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}