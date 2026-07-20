use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileCreatedEvent {
    pub file_path: String,
    pub file_name: String,
    pub detected_at: String,
}
