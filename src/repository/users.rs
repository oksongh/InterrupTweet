use sqlx::SqlitePool;

use crate::usecases::common::FatalError;

pub async fn user_exists(user_id: i64, pool: &SqlitePool) -> Result<bool, FatalError> {
    let exists = sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM users WHERE user_id = ?)")
        .bind(user_id)
        .fetch_one(pool)
        .await?;

    Ok(exists)
}
