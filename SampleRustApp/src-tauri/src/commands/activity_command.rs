use tauri::State;
use tokio::sync::watch;

use crate::{
    models::activity_log::ActivityLog,
    repositories::activity_repository::ActivityRepository,
    services::activity_monitor::run_activity_monitor,
    states::activity_monitor_state::{
        ActivityMonitorState,
        MonitorCommand,
    },
};

const MIN_INTERVAL_SECONDS: u64 = 1;
const MAX_INTERVAL_SECONDS: u64 = 3_600;

fn validate_interval(
    interval_seconds: u64,
) -> Result<(), String> {
    if !(MIN_INTERVAL_SECONDS..=MAX_INTERVAL_SECONDS)
        .contains(&interval_seconds)
    {
        return Err(format!(
            "記録間隔は{}秒から{}秒で指定してください。",
            MIN_INTERVAL_SECONDS,
            MAX_INTERVAL_SECONDS
        ));
    }

    Ok(())
}

#[tauri::command]
pub async fn start_activity_monitor(
    interval_seconds: u64,
    repository: State<'_, ActivityRepository>,
    state: State<'_, ActivityMonitorState>,
) -> Result<(), String> {
    validate_interval(interval_seconds)?;

    let repository = repository.inner().clone();

    let (sender, receiver) = watch::channel(
        MonitorCommand::ChangeInterval(interval_seconds),
    );

    state.start(sender)?;

    tauri::async_runtime::spawn(async move {
        run_activity_monitor(
            repository,
            interval_seconds,
            receiver,
        )
        .await;
    });

    Ok(())
}

#[tauri::command]
pub async fn change_activity_interval(
    interval_seconds: u64,
    state: State<'_, ActivityMonitorState>,
) -> Result<(), String> {
    validate_interval(interval_seconds)?;

    state.send(
        MonitorCommand::ChangeInterval(interval_seconds),
    )
}

#[tauri::command]
pub async fn stop_activity_monitor(
    state: State<'_, ActivityMonitorState>,
) -> Result<(), String> {
    state.send(MonitorCommand::Stop)?;
    state.clear()?;

    Ok(())
}

#[tauri::command]
pub async fn is_activity_monitor_running(
    state: State<'_, ActivityMonitorState>,
) -> Result<bool, String> {
    state.is_running()
}

#[tauri::command]
pub async fn get_activity_logs(
    limit: i64,
    repository: State<'_, ActivityRepository>,
) -> Result<Vec<ActivityLog>, String> {
    let normalized_limit = limit.clamp(1, 1_000);

    repository
        .find_latest(normalized_limit)
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn delete_all_activity_logs(
    repository: State<'_, ActivityRepository>,
) -> Result<u64, String> {
    repository
        .delete_all()
        .await
        .map_err(|error| error.to_string())
}