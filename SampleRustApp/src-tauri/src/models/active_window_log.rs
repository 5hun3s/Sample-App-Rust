use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ActiveWindowLog {
    pub id: i64,
    pub window_title: String,
    pub process_id: i64,
    pub recorded_at: String,
}
