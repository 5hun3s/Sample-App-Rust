use std::{
    path::Path,
    time::Duration,
};

use tokio::time::sleep;

const CHECK_INTERVAL_SECONDS: u64 = 1;
const REQUIRED_STABLE_CHECKS: u32 = 3;
const MAX_CHECKS: u32 = 60;

pub async fn wait_until_file_is_stable(
    file_path: &Path,
) -> Result<(), String> {
    let mut previous_size: Option<u64> = None;
    let mut stable_count = 0_u32;

    for _ in 0..MAX_CHECKS {
        if !file_path.exists() {
            return Err(
                "安定確認中にファイルが存在しなくなりました。"
                    .to_string(),
            );
        }

        if !file_path.is_file() {
            return Err(
                "検知したパスはファイルではありません。"
                    .to_string(),
            );
        }

        let metadata = match tokio::fs::metadata(file_path).await {
            Ok(metadata) => metadata,
            Err(_) => {
                sleep(Duration::from_secs(
                    CHECK_INTERVAL_SECONDS,
                ))
                .await;

                continue;
            }
        };

        let current_size = metadata.len();

        match previous_size {
            Some(size) if size == current_size => {
                stable_count += 1;
            }

            _ => {
                stable_count = 0;
                previous_size = Some(current_size);
            }
        }

        if stable_count >= REQUIRED_STABLE_CHECKS {
            return Ok(());
        }

        sleep(Duration::from_secs(
            CHECK_INTERVAL_SECONDS,
        ))
        .await;
    }

    Err("ファイルのコピー完了を確認できませんでした。".to_string())
}