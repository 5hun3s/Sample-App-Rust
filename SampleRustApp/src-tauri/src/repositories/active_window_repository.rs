use sqlx::SqlitePool;

use crate::models::active_window_log::ActiveWindowLog;

#[derive(Clone)]
pub struct ActiveWindowRepository {
    pool: SqlitePool,
}

impl ActiveWindowRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        window_title: &str,
        process_id: u32,
    ) -> Result<i64, sqlx::Error> {
        let result = sqlx::query(
            r#"
            INSERT INTO active_window_logs (
                window_title,
                process_id
            )
            VALUES (?, ?)
            "#,
        )
        .bind(window_title)
        .bind(i64::from(process_id))
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    pub async fn find_all(
        &self,
    ) -> Result<Vec<ActiveWindowLog>, sqlx::Error> {
        sqlx::query_as::<_, ActiveWindowLog>(
            r#"
            SELECT
                id,
                window_title,
                process_id,
                recorded_at
            FROM active_window_logs
            ORDER BY id DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await
    }
}