use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_spotify_state(
    pool: &PgPool,
    user_id: Uuid,
    csrf_token: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO spotify_oauth_state (csrf_token, user_id) VALUES ($1, $2)")
        .bind(csrf_token)
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Atomically deletes and returns the user_id for the given CSRF token.
/// Returns None if the token doesn't exist or has expired.
pub async fn consume_spotify_state(
    pool: &PgPool,
    csrf_token: &str,
) -> Result<Option<Uuid>, sqlx::Error> {
    sqlx::query_scalar::<_, Uuid>(
        "DELETE FROM spotify_oauth_state
         WHERE csrf_token = $1 AND expires_at > NOW()
         RETURNING user_id",
    )
    .bind(csrf_token)
    .fetch_optional(pool)
    .await
}
