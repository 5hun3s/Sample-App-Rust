use sqlx::{Pool, Sqlite, sqlite::SqlitePoolOptions};

pub async fn create_pool() -> Pool<Sqlite> {
    SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:sample.db")
        .await
        .unwrap()
}