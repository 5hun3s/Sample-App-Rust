use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ActivityLog {
    pub id: i64,

    pub window_title: String,
    pub process_id: i64,
    pub process_name: Option<String>,
    pub executable_path: Option<String>,
    pub window_class: Option<String>,

    pub window_x: Option<i64>,
    pub window_y: Option<i64>,
    pub window_width: Option<i64>,
    pub window_height: Option<i64>,

    pub is_maximized: bool,
    pub is_minimized: bool,

    pub idle_seconds: i64,

    pub started_at: String,
    pub ended_at: String,
    pub duration_seconds: i64,
}