use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Deserialize, Debug)]
pub struct NewUser {
    name: String,
    at_id: String,
    birthday: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct User {
    id: i64,
    name: String,
    at_id: String,
    birthday: Option<String>,
}
// crud
// c : succ :(statuscode::created, json<string>) ,fail(statuscode::bad_request, json<string>)
// r : (statuscode::, json<array<user>>) ,fail(statuscode::not_found, json<string>)
// u : (statuscode, json<string>)
// d : (statuscode, json<string>)
pub async fn create_new_user(
    State(pool): State<SqlitePool>,
    Json(NewUser {
        name,
        at_id,
        birthday,
    }): Json<NewUser>,
) -> impl IntoResponse {
    // validate input
    if at_id.len() < 3 {
        return (
            StatusCode::BAD_REQUEST,
            Json("at_id should be at least 3 characters long".to_string()),
        )
            .into_response();
    }

    if let Some(birthday) = &birthday {
        if NaiveDate::parse_from_str(birthday, "%Y-%m-%d").is_err() {
            return (
                StatusCode::BAD_REQUEST,
                Json("birthday should be in format YYYY-MM-DD".to_string()),
            )
                .into_response();
        }
    }

    let result = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (name, at_id, birthday)
        VALUES (?,?,?)
        RETURNING id,name, at_id, birthday
        "#,
        name,
        at_id,
        birthday
    )
    .fetch_one(&pool)
    .await;

    match result {
        Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
        Err(_) => (
            StatusCode::BAD_REQUEST,
            Json("failed to create user".to_string()),
        )
            .into_response(),
    }
}

// get all users
pub async fn get_users(State(pool): State<SqlitePool>) -> impl IntoResponse {
    let result = sqlx::query_as!(User, "SELECT id, name, at_id, birthday FROM users")
        .fetch_all(&pool)
        .await;

    match result {
        Ok(users) => (StatusCode::OK, Json(users)).into_response(),
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json("failed to get users".to_string()),
        )
            .into_response(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test(migrations = "./migrations")]
    async fn test_create_new_user_birthday_some(pool: SqlitePool) {
        let new_user = NewUser {
            name: "TestUser".to_string(),
            at_id: "test_user".to_string(),
            birthday: Some("2023-01-01".to_string()),
        };
        let response = create_new_user(State(pool.clone()), Json(new_user)).await;
        assert_eq!(response.into_response().status(), StatusCode::CREATED);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_create_new_user_birthday_none(pool: SqlitePool) {
        let new_user = NewUser {
            name: "TestUser".to_string(),
            at_id: "test_user".to_string(),
            birthday: None,
        };
        let response = create_new_user(State(pool.clone()), Json(new_user)).await;
        assert_eq!(response.into_response().status(), StatusCode::CREATED);
    }
    #[sqlx::test(migrations = "./migrations")]
    async fn test_create_new_user_birthday_invalid(pool: SqlitePool) {
        let new_user = NewUser {
            name: "TestUser".to_string(),
            at_id: "test_user".to_string(),
            birthday: Some("2023-13-01".to_string()),
        };
        let response = create_new_user(State(pool.clone()), Json(new_user)).await;
        assert_eq!(response.into_response().status(), StatusCode::BAD_REQUEST);
    }
}
