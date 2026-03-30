use crate::types::User;
use rand_core::{OsRng, RngCore};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_session(pool: &PgPool, user_id: Uuid) -> Result<String, sqlx::Error> {
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    let token: String = bytes.iter().map(|b| format!("{:02x}", b)).collect();

    sqlx::query("INSERT INTO sessions (token, user_id) VALUES ($1, $2)")
        .bind(&token)
        .bind(user_id)
        .execute(pool)
        .await?;

    Ok(token)
}

pub async fn get_session_user(pool: &PgPool, token: &str) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(
        "SELECT u.*
         FROM users u
         JOIN sessions s ON s.user_id = u.id
         WHERE s.token = $1 AND s.expires_at > NOW()",
    )
    .bind(token)
    .fetch_optional(pool)
    .await
}

pub async fn delete_session(pool: &PgPool, token: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM sessions WHERE token = $1")
        .bind(token)
        .execute(pool)
        .await?;
    Ok(())
}
