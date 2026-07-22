mod commands;
mod models;
mod platform;
mod repositories;
mod states;
mod services;

use std::fs;

use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use tauri::Manager;

use commands::active_window_command::{
    get_active_window_logs,
    start_active_window_monitor,
    stop_active_window_monitor,
    update_active_window_interval,
};
use repositories::{
    activity_repository::ActivityRepository,
    note_repository::NoteRepository,
};
use states::active_window_monitor_state::ActiveWindowMonitorState;
use states::activity_monitor_state::ActivityMonitorState;
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

            let note_repository = NoteRepository::new(pool.clone());
            let activity_repository =
                ActivityRepository::new(pool);

            app.manage(note_repository);
            app.manage(activity_repository);
            app.manage(ActivityMonitorState::new());

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
            
            commands::activity_command::start_activity_monitor,
            commands::activity_command::change_activity_interval,
            commands::activity_command::stop_activity_monitor,
            commands::activity_command::is_activity_monitor_running,
            commands::activity_command::get_activity_logs,
            commands::activity_command::delete_all_activity_logs,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}