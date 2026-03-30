pub mod sessions;
pub mod spotify_state;
pub mod users;

pub type DbPool = sqlx::PgPool;

pub async fn connect(database_url: &str) -> DbPool {
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("Failed to connect to database")
}