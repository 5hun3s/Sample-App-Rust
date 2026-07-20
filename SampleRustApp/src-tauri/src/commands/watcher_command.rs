use std::{
    path::{
        Path,
        PathBuf,
    },
    sync::Arc,
};

use chrono::Local;
use notify::{
    event::CreateKind,
    Config,
    Event,
    EventKind,
    RecommendedWatcher,
    RecursiveMode,
    Watcher,
};
use tauri::{
    AppHandle,
    Emitter,
    State,
};

use crate::{
    models::watcher_event::FileCreatedEvent,
    services::{
        bat_executor_service::execute_bat,
        file_stability_service::wait_until_file_is_stable,
    },
    states::watcher_state::WatcherState,
};

const FILE_CREATED_EVENT_NAME: &str = "file-created";
const BAT_STARTED_EVENT_NAME: &str = "bat-started";
const BAT_FINISHED_EVENT_NAME: &str = "bat-finished";
const WATCHER_ERROR_EVENT_NAME: &str = "watcher-error";

#[tauri::command]
pub async fn start_folder_watcher(
    watch_directory: String,
    bat_path: String,
    app_handle: AppHandle,
    watcher_state: State<'_, WatcherState>,
) -> Result<(), String> {
    let watch_directory = normalize_directory(
        Path::new(&watch_directory),
    )?;

    let bat_path = normalize_bat_path(
        Path::new(&bat_path),
    )?;

    /*
     * notifyのコールバックは別スレッドから呼ばれる可能性がある。
     * 複数回使う値なのでArcで共有する。
     */
    let watch_directory = Arc::new(watch_directory);
    let bat_path = Arc::new(bat_path);

    let callback_app_handle = app_handle.clone();
    let callback_watch_directory =
        Arc::clone(&watch_directory);
    let callback_bat_path = Arc::clone(&bat_path);

    let mut watcher = RecommendedWatcher::new(
        move |result: notify::Result<Event>| {
            let event = match result {
                Ok(event) => event,

                Err(error) => {
                    let _ = callback_app_handle.emit(
                        WATCHER_ERROR_EVENT_NAME,
                        format!(
                            "フォルダ監視エラー: {error}"
                        ),
                    );

                    return;
                }
            };

            if !is_file_created_event(&event.kind) {
                return;
            }

            for path in event.paths {
                /*
                 * フォルダ作成イベントを除外する。
                 *
                 * イベント直後はis_file()がfalseになることもあるので、
                 * existsしていて明確にディレクトリの場合だけ除外する。
                 */
                if path.is_dir() {
                    continue;
                }

                let app_handle =
                    callback_app_handle.clone();

                let watch_directory =
                    Arc::clone(&callback_watch_directory);

                let bat_path =
                    Arc::clone(&callback_bat_path);

                /*
                 * notifyのコールバック内で長時間待たない。
                 * 非同期タスクを起動し、安定確認とBAT実行を行う。
                 */
                tauri::async_runtime::spawn(async move {
                    process_created_file(
                        app_handle,
                        path,
                        watch_directory.as_path(),
                        bat_path.as_path(),
                    )
                    .await;
                });
            }
        },
        Config::default(),
    )
    .map_err(|error| {
        format!("Watcherの作成に失敗しました: {error}")
    })?;

    watcher
        .watch(
            watch_directory.as_path(),
            RecursiveMode::NonRecursive,
        )
        .map_err(|error| {
            format!(
                "フォルダ監視の開始に失敗しました: {error}"
            )
        })?;

    watcher_state.set_watcher(watcher)?;

    Ok(())
}

#[tauri::command]
pub async fn stop_folder_watcher(
    watcher_state: State<'_, WatcherState>,
) -> Result<bool, String> {
    watcher_state.stop()
}

#[tauri::command]
pub async fn get_watcher_status(
    watcher_state: State<'_, WatcherState>,
) -> Result<bool, String> {
    watcher_state.is_running()
}

async fn process_created_file(
    app_handle: AppHandle,
    file_path: PathBuf,
    _watch_directory: &Path,
    bat_path: &Path,
) {
    let file_name = file_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("")
        .to_string();

    let detected_event = FileCreatedEvent {
        file_path: file_path
            .to_string_lossy()
            .into_owned(),

        file_name,
        detected_at: Local::now().to_rfc3339(),
    };

    let _ = app_handle.emit(
        FILE_CREATED_EVENT_NAME,
        detected_event,
    );

    /*
     * ファイルサイズが安定するまで待つ。
     */
    if let Err(error) =
        wait_until_file_is_stable(&file_path).await
    {
        let _ = app_handle.emit(
            WATCHER_ERROR_EVENT_NAME,
            format!(
                "{}: {error}",
                file_path.display()
            ),
        );

        return;
    }

    let _ = app_handle.emit(
        BAT_STARTED_EVENT_NAME,
        file_path.to_string_lossy().into_owned(),
    );

    match execute_bat(bat_path, &file_path).await {
        Ok(result) => {
            let _ = app_handle.emit(
                BAT_FINISHED_EVENT_NAME,
                result,
            );
        }

        Err(error) => {
            let _ = app_handle.emit(
                WATCHER_ERROR_EVENT_NAME,
                format!(
                    "{}: {error}",
                    file_path.display()
                ),
            );
        }
    }
}

fn is_file_created_event(event_kind: &EventKind) -> bool {
    matches!(
        event_kind,
        EventKind::Create(CreateKind::File)
            | EventKind::Create(CreateKind::Any)
            | EventKind::Create(CreateKind::Other)
    )
}

fn normalize_directory(path: &Path) -> Result<PathBuf, String> {
    if !path.exists() {
        return Err(format!(
            "監視フォルダが存在しません: {}",
            path.display()
        ));
    }

    if !path.is_dir() {
        return Err(format!(
            "監視対象はフォルダではありません: {}",
            path.display()
        ));
    }

    path.canonicalize().map_err(|error| {
        format!(
            "監視フォルダの絶対パス取得に失敗しました: {error}"
        )
    })
}

fn normalize_bat_path(path: &Path) -> Result<PathBuf, String> {
    if !path.exists() {
        return Err(format!(
            "BATファイルが存在しません: {}",
            path.display()
        ));
    }

    if !path.is_file() {
        return Err(format!(
            "BATのパスはファイルではありません: {}",
            path.display()
        ));
    }

    let extension = path
        .extension()
        .and_then(|extension| extension.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();

    if extension != "bat" && extension != "cmd" {
        return Err(
            "BATファイルには.batまたは.cmdを指定してください。"
                .to_string(),
        );
    }

    path.canonicalize().map_err(|error| {
        format!(
            "BATファイルの絶対パス取得に失敗しました: {error}"
        )
    })
}