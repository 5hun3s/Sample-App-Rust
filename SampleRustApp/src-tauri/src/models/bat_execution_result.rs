use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatExecutionResult {
    pub file_path: String,
    pub bat_path: String,

    pub success: bool,
    pub exit_code: Option<i32>,

    pub stdout: String,
    pub stderr: String,

    pub started_at: String,
    pub finished_at: String,
    pub duration_milliseconds: u128,
}
