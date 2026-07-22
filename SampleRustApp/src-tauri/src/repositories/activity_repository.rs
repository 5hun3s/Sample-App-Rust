use sqlx::SqlitePool;

use crate::{
    models::activity_log::ActivityLog,
    platform::active_window::ActiveWindowInfo,
};

#[derive(Clone)]
pub struct ActivityRepository {
    pool: SqlitePool,
}

impl ActivityRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_session(
        &self,
        info: &ActiveWindowInfo,
    ) -> Result<i64, sqlx::Error> {
        let result = sqlx::query(
            r#"
            INSERT INTO activity_logs (
                window_title,
                process_id,
                process_name,
                executable_path,
                window_class,

                window_x,
                window_y,
                window_width,
                window_height,

                is_maximized,
                is_minimized,
                idle_seconds,

                started_at,
                ended_at,
                duration_seconds
            )
            VALUES (
                ?, ?, ?, ?, ?,
                ?, ?, ?, ?,
                ?, ?, ?,
                CURRENT_TIMESTAMP,
                CURRENT_TIMESTAMP,
                0
            )
            "#,
        )
        .bind(&info.window_title)
        .bind(i64::from(info.process_id))
        .bind(&info.process_name)
        .bind(&info.executable_path)
        .bind(&info.window_class)
        .bind(info.window_x.map(i64::from))
        .bind(info.window_y.map(i64::from))
        .bind(info.window_width.map(i64::from))
        .bind(info.window_height.map(i64::from))
        .bind(info.is_maximized)
        .bind(info.is_minimized)
        .bind(info.idle_seconds as i64)
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    pub async fn update_session(
        &self,
        id: i64,
        info: &ActiveWindowInfo,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query(
            r#"
            UPDATE activity_logs
            SET
                process_id = ?,
                window_class = ?,

                window_x = ?,
                window_y = ?,
                window_width = ?,
                window_height = ?,

                is_maximized = ?,
                is_minimized = ?,
                idle_seconds = ?,

                ended_at = CURRENT_TIMESTAMP,
                duration_seconds =
                    CAST(
                        strftime('%s', CURRENT_TIMESTAMP)
                        - strftime('%s', started_at)
                        AS INTEGER
                    )
            WHERE id = ?
            "#,
        )
        .bind(i64::from(info.process_id))
        .bind(&info.window_class)
        .bind(info.window_x.map(i64::from))
        .bind(info.window_y.map(i64::from))
        .bind(info.window_width.map(i64::from))
        .bind(info.window_height.map(i64::from))
        .bind(info.is_maximized)
        .bind(info.is_minimized)
        .bind(info.idle_seconds as i64)
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn find_latest(
        &self,
        limit: i64,
    ) -> Result<Vec<ActivityLog>, sqlx::Error> {
        sqlx::query_as::<_, ActivityLog>(
            r#"
            SELECT
                id,
                window_title,
                process_id,
                process_name,
                executable_path,
                window_class,

                window_x,
                window_y,
                window_width,
                window_height,

                is_maximized,
                is_minimized,
                idle_seconds,

                started_at,
                ended_at,
                duration_seconds
            FROM activity_logs
            ORDER BY started_at DESC
            LIMIT ?
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn delete_all(
        &self,
    ) -> Result<u64, sqlx::Error> {
        let result = sqlx::query(
            "DELETE FROM activity_logs",
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }
}