use std::{
    path::Path,
    time::Instant,
};

use chrono::Local;

use crate::models::bat_execution_result::BatExecutionResult;

pub async fn execute_bat(
    bat_path: &Path,
    target_file_path: &Path,
) -> Result<BatExecutionResult, String> {
    validate_bat_path(bat_path)?;
    validate_target_file(target_file_path)?;

    execute_bat_platform(bat_path, target_file_path).await
}

fn validate_bat_path(bat_path: &Path) -> Result<(), String> {
    if !bat_path.exists() {
        return Err(format!(
            "BATファイルが存在しません: {}",
            bat_path.display()
        ));
    }

    if !bat_path.is_file() {
        return Err(format!(
            "BATとして指定されたパスはファイルではありません: {}",
            bat_path.display()
        ));
    }

    let extension = bat_path
        .extension()
        .and_then(|extension| extension.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();

    if extension != "bat" && extension != "cmd" {
        return Err(
            "実行可能なのは.batまたは.cmdファイルだけです。"
                .to_string(),
        );
    }

    Ok(())
}

fn validate_target_file(
    target_file_path: &Path,
) -> Result<(), String> {
    if !target_file_path.exists() {
        return Err(format!(
            "対象ファイルが存在しません: {}",
            target_file_path.display()
        ));
    }

    if !target_file_path.is_file() {
        return Err(format!(
            "対象パスはファイルではありません: {}",
            target_file_path.display()
        ));
    }

    Ok(())
}

#[cfg(target_os = "windows")]
async fn execute_bat_platform(
    bat_path: &Path,
    target_file_path: &Path,
) -> Result<BatExecutionResult, String> {
    use tokio::process::Command;

    let started_at = Local::now();
    let started_instant = Instant::now();

    /*
     * 現在のMVPでは、BATへ対象ファイルの絶対パスを
     * 第1引数として渡す。
     *
     * 例:
     * import-data.bat "C:\Import Files\sample.csv"
     */
    let output = Command::new("cmd.exe")
        .arg("/D")
        .arg("/S")
        .arg("/C")
        .arg(bat_path.as_os_str())
        .arg(target_file_path.as_os_str())
        .output()
        .await
        .map_err(|error| {
            format!("BATの起動に失敗しました: {error}")
        })?;

    let finished_at = Local::now();
    let duration = started_instant.elapsed();

    let exit_code = output.status.code();
    let success = output.status.success();

    /*
     * WindowsのBAT出力はCP932の場合がある。
     * 現段階ではUTF-8として変換し、不正部分は
     * 置換文字として表示する。
     *
     * CP932対応は次の段階で追加する。
     */
    let stdout =
        String::from_utf8_lossy(&output.stdout).into_owned();

    let stderr =
        String::from_utf8_lossy(&output.stderr).into_owned();

    Ok(BatExecutionResult {
        file_path: target_file_path
            .to_string_lossy()
            .into_owned(),

        bat_path: bat_path
            .to_string_lossy()
            .into_owned(),

        success,
        exit_code,
        stdout,
        stderr,

        started_at: started_at.to_rfc3339(),
        finished_at: finished_at.to_rfc3339(),
        duration_milliseconds: duration.as_millis(),
    })
}

/*
 * UbuntuのDev ContainerではWindowsのBATを実行できないため、
 * cargo checkを通すためのLinux用実装を用意する。
 */
#[cfg(not(target_os = "windows"))]
async fn execute_bat_platform(
    _bat_path: &Path,
    _target_file_path: &Path,
) -> Result<BatExecutionResult, String> {
    Err(
        "BATファイルの実行はWindows版でのみ利用できます。"
            .to_string(),
    )
}