use sqlx::SqlitePool;

use crate::{domain, usecases::fatal_error::FatalError};

pub async fn user_exists(
    user_id: &domain::users::UserId,
    pool: &SqlitePool,
) -> Result<bool, FatalError> {
    let exists = sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM users WHERE user_id = ?)")
        .bind(user_id.value().to_string())
        .fetch_one(pool)
        .await?;

    Ok(exists)
}
