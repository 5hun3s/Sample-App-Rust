use std::time::Duration;

use tokio::{
    sync::watch,
    time::{self, MissedTickBehavior},
};

use crate::{
    platform::active_window::get_active_window_info,
    repositories::activity_repository::ActivityRepository,
    states::activity_monitor_state::MonitorCommand,
};

pub async fn run_activity_monitor(
    repository: ActivityRepository,
    interval_seconds: u64,
    mut receiver: watch::Receiver<MonitorCommand>,
) {
    let mut ticker = create_interval(interval_seconds);

    loop {
        tokio::select! {
            _ = ticker.tick() => {
                record_current_activity(&repository).await;
            }

            changed = receiver.changed() => {
                if changed.is_err() {
                    break;
                }

                let command = receiver.borrow().clone();

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

    // タイトルのないウィンドウは記録しない
    if info.window_title.trim().is_empty() {
        return;
    }

    if let Err(error) = repository.create(&info).await {
        eprintln!(
            "アクティビティ保存エラー: {error}"
        );
    }
}