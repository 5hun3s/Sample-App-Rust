use std::time::Duration;

use tokio::{
    sync::watch,
    time::{self, MissedTickBehavior},
};

use crate::{
    platform::active_window::{
        get_active_window_info,
        ActiveWindowInfo,
    },
    repositories::activity_repository::ActivityRepository,
    states::activity_monitor_state::MonitorCommand,
};

#[derive(Debug, Clone)]
struct CurrentActivitySession {
    id: i64,
    process_name: Option<String>,
    executable_path: Option<String>,
    window_title: String,
}

pub async fn run_activity_monitor(
    repository: ActivityRepository,
    interval_seconds: u64,
    mut receiver: watch::Receiver<MonitorCommand>,
) {
    let mut ticker = create_interval(interval_seconds);

    /*
     * 現在更新中のDBレコードを保持する。
     *
     * 最初はまだ何も記録していないのでNone。
     */
    let mut current_session:
        Option<CurrentActivitySession> = None;

    loop {
        tokio::select! {
            _ = ticker.tick() => {
                record_current_activity(
                    &repository,
                    &mut current_session,
                )
                .await;
            }

            changed = receiver.changed() => {
                if changed.is_err() {
                    break;
                }

                let command =
                    receiver.borrow().clone();

                match command {
                    MonitorCommand::ChangeInterval(seconds) => {
                        ticker = create_interval(seconds);
                    }

                    MonitorCommand::Stop => {
                        break;
                    }
                }
            }
        }
    }
}

fn create_interval(
    interval_seconds: u64,
) -> time::Interval {
    let mut ticker = time::interval(
        Duration::from_secs(interval_seconds),
    );

    ticker.set_missed_tick_behavior(
        MissedTickBehavior::Skip,
    );

    ticker
}

async fn record_current_activity(
    repository: &ActivityRepository,
    current_session: &mut Option<CurrentActivitySession>,
) {
    let info = match get_active_window_info() {
        Ok(Some(info)) => info,

        Ok(None) => {
            return;
        }

        Err(error) => {
            eprintln!(
                "アクティブウィンドウ取得エラー: {error}"
            );
            return;
        }
    };

    if info.window_title.trim().is_empty() {
        return;
    }

    /*
     * 現在のセッションが存在し、
     * 同じウィンドウなら既存レコードを更新する。
     */
    if let Some(session) = current_session.as_ref() {
        if is_same_activity(session, &info) {
            if let Err(error) = repository
                .update_session(session.id, &info)
                .await
            {
                eprintln!(
                    "アクティビティ更新エラー: {error}"
                );
            }

            return;
        }
    }

    /*
     * ウィンドウが変わった場合は、
     * 新しいDBレコードを作成する。
     */
    match repository.create_session(&info).await {
        Ok(id) => {
            *current_session =
                Some(CurrentActivitySession {
                    id,
                    process_name:
                        info.process_name.clone(),
                    executable_path:
                        info.executable_path.clone(),
                    window_title:
                        info.window_title.clone(),
                });
        }

        Err(error) => {
            eprintln!(
                "アクティビティ作成エラー: {error}"
            );
        }
    }
}

fn is_same_activity(
    current: &CurrentActivitySession,
    info: &ActiveWindowInfo,
) -> bool {
    current.process_name == info.process_name
        && current.executable_path == info.executable_path
        && current.window_title == info.window_title
}