use sqlx::{sqlite::SqliteQueryResult, SqlitePool};

use crate::models::note::Note;

pub struct NoteRepository {
    pool: SqlitePool,
}

impl NoteRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        title: &str,
        content: &str,
    ) -> Result<i64, sqlx::Error> {
        let result: SqliteQueryResult = sqlx::query(
            r#"
            INSERT INTO notes (
                title,
                content
            )
            VALUES (?, ?)
            "#,
        )
        .bind(title)
        .bind(content)
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    pub async fn find_all(&self) -> Result<Vec<Note>, sqlx::Error> {
        let notes = sqlx::query_as::<_, Note>(
            r#"
            SELECT
                id,
                title,
                content,
                created_at
            FROM notes
            ORDER BY id DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(notes)
    }

    pub async fn find_by_id(
        &self,
        id: i64,
    ) -> Result<Option<Note>, sqlx::Error> {
        let note = sqlx::query_as::<_, Note>(
            r#"
            SELECT
                id,
                title,
                content,
                created_at
            FROM notes
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(note)
    }

    pub async fn update(
        &self,
        id: i64,
        title: &str,
        content: &str,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query(
            r#"
            UPDATE notes
            SET
                title = ?,
                content = ?
            WHERE id = ?
            "#,
        )
        .bind(title)
        .bind(content)
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn delete(&self, id: i64) -> Result<bool, sqlx::Error> {
        let result = sqlx::query(
            r#"
            DELETE FROM notes
            WHERE id = ?
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}