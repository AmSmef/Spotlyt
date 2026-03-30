use crate::types::User;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_user(
    pool: &PgPool,
    username: &str,
    password_hash: &str,
    display_name: &str,
    email: Option<&str>,
) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>(
        "INSERT INTO users (username, password_hash, display_name, email)
         VALUES ($1, $2, $3, $4)
         RETURNING *",
    )
    .bind(username)
    .bind(password_hash)
    .bind(display_name)
    .bind(email)
    .fetch_one(pool)
    .await
}

pub async fn find_by_username(pool: &PgPool, username: &str) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(username)
        .fetch_optional(pool)
        .await
}

pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn find_or_create_by_google_id(
    pool: &PgPool,
    google_id: &str,
    email: &str,
    display_name: &str,
) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>(
        "INSERT INTO users (google_id, email, display_name)
         VALUES ($1, $2, $3)
         ON CONFLICT (google_id) DO UPDATE SET updated_at = NOW()
         RETURNING *",
    )
    .bind(google_id)
    .bind(email)
    .bind(display_name)
    .fetch_one(pool)
    .await
}

/// Updates the Spotify access token and expiry. Only overwrites the refresh token
/// when a new one is provided (Spotify only re-issues refresh tokens occasionally).
pub async fn update_spotify_tokens(
    pool: &PgPool,
    user_id: Uuid,
    access_token: &str,
    refresh_token: Option<&str>,
    expires_at: DateTime<Utc>,
) -> Result<(), sqlx::Error> {
    if let Some(refresh) = refresh_token {
        sqlx::query(
            "UPDATE users
             SET spotify_access_token = $1,
                 spotify_refresh_token = $2,
                 spotify_token_expires_at = $3,
                 updated_at = NOW()
             WHERE id = $4",
        )
        .bind(access_token)
        .bind(refresh)
        .bind(expires_at)
        .bind(user_id)
        .execute(pool)
        .await?;
    } else {
        sqlx::query(
            "UPDATE users
             SET spotify_access_token = $1,
                 spotify_token_expires_at = $2,
                 updated_at = NOW()
             WHERE id = $3",
        )
        .bind(access_token)
        .bind(expires_at)
        .bind(user_id)
        .execute(pool)
        .await?;
    }
    Ok(())
}

pub async fn clear_spotify_tokens(pool: &PgPool, user_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE users
         SET spotify_refresh_token = NULL,
             spotify_access_token = NULL,
             spotify_token_expires_at = NULL,
             updated_at = NOW()
         WHERE id = $1",
    )
    .bind(user_id)
    .execute(pool)
    .await?;
    Ok(())
}
