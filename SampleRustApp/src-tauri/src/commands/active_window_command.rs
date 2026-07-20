use std::time::Duration;

use tauri::State;
use tokio::{
    sync::watch,
    time::{self, MissedTickBehavior},
};

use crate::{
    models::active_window_log::ActiveWindowLog,
    platform::active_window::get_active_window,
    repositories::active_window_repository::ActiveWindowRepository,
    states::active_window_monitor_state::{
        ActiveWindowMonitorState,
        MonitorCommand,
    },
};

const MINIMUM_INTERVAL_SECONDS: u64 = 1;
const MAXIMUM_INTERVAL_SECONDS: u64 = 3600;

fn validate_interval(interval_seconds: u64) -> Result<(), String> {
    if !(MINIMUM_INTERVAL_SECONDS..=MAXIMUM_INTERVAL_SECONDS)
        .contains(&interval_seconds)
    {
        return Err(format!(
            "記録間隔は{}秒から{}秒の間で指定してください。",
            MINIMUM_INTERVAL_SECONDS,
            MAXIMUM_INTERVAL_SECONDS
        ));
    }

    Ok(())
}

#[tauri::command]
pub async fn start_active_window_monitor(
    interval_seconds: u64,
    repository: State<'_, ActiveWindowRepository>,
    monitor_state: State<'_, ActiveWindowMonitorState>,
) -> Result<(), String> {
    validate_interval(interval_seconds)?;

    let repository = repository.inner().clone();

    let initial_command =
        MonitorCommand::UpdateInterval(interval_seconds);

    let (sender, mut receiver) =
        watch::channel(initial_command);

    monitor_state.replace_sender(sender)?;

    tauri::async_runtime::spawn(async move {
        let mut current_interval_seconds = interval_seconds;

        let mut ticker = time::interval(
            Duration::from_secs(current_interval_seconds),
        );

        /*
         * 処理が遅れた場合に、遅れた分を連続実行せず、
         * 次回周期まで飛ばす。
         */
        ticker.set_missed_tick_behavior(
            MissedTickBehavior::Skip,
        );

        /*
         * time::intervalは最初のtickが即時完了する。
         * そのため、開始直後にも1件記録される。
         */
        loop {
            tokio::select! {
                _ = ticker.tick() => {
                    match get_active_window() {
                        Ok(Some(window)) => {
                            /*
                             * タイトルなしのウィンドウを保存したくない場合は
                             * 空文字を除外する。
                             */
                            if window.title.trim().is_empty() {
                                continue;
                            }

                            if let Err(error) = repository
                                .create(
                                    &window.title,
                                    window.process_id,
                                )
                                .await
                            {
                                eprintln!(
                                    "アクティブウィンドウ保存エラー: {error}"
                                );
                            }
                        }

                        Ok(None) => {
                            // アクティブウィンドウがないため何もしない
                        }

                        Err(error) => {
                            eprintln!(
                                "アクティブウィンドウ取得エラー: {error}"
                            );
                        }
                    }
                }

                changed = receiver.changed() => {
                    if changed.is_err() {
                        break;
                    }

                    let command = receiver.borrow().clone();

                    match command {
                        MonitorCommand::UpdateInterval(new_interval) => {
                            current_interval_seconds = new_interval;

                            ticker = time::interval(
                                Duration::from_secs(
                                    current_interval_seconds,
                                ),
                            );

                            ticker.set_missed_tick_behavior(
                                MissedTickBehavior::Skip,
                            );
                        }

                        MonitorCommand::Stop => {
                            break;
                        }
                    }
                }
            }
        }
    });

    Ok(())
}

#[tauri::command]
pub async fn update_active_window_interval(
    interval_seconds: u64,
    monitor_state: State<'_, ActiveWindowMonitorState>,
) -> Result<(), String> {
    validate_interval(interval_seconds)?;

    monitor_state.send_command(
        MonitorCommand::UpdateInterval(interval_seconds),
    )
}

#[tauri::command]
pub async fn stop_active_window_monitor(
    monitor_state: State<'_, ActiveWindowMonitorState>,
) -> Result<(), String> {
    monitor_state.send_command(MonitorCommand::Stop)
}

#[tauri::command]
pub async fn get_active_window_logs(
    repository: State<'_, ActiveWindowRepository>,
) -> Result<Vec<ActiveWindowLog>, String> {
    repository
        .find_all()
        .await
        .map_err(|error| {
            eprintln!("ログ一覧取得エラー: {error}");
            "アクティブウィンドウ履歴の取得に失敗しました。".to_string()
        })
}