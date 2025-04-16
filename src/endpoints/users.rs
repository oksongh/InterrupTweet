use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    routing::post,
    Json, Router,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite, SqlitePool};

#[derive(Deserialize, Debug, Clone)]
pub struct NewUser {
    name: String,
    at_id: String,
    birthday: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
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

/// get user by id
pub async fn get_user_by_id(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let result = sqlx::query_as!(
        User,
        "SELECT id, name, at_id, birthday FROM users WHERE id = ?",
        id
    )
    .fetch_one(&pool)
    .await;

    match result {
        Ok(user) => (StatusCode::OK, Json(user)).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, Json("user not found".to_string())).into_response(),
    }
}

/// get all users
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

fn app() -> Router<Pool<Sqlite>> {
    Router::new()
        .route("/user", post(create_new_user).get(get_users))
        .route("/user/{:id}", get(get_user_by_id))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http};
    use http_body_util::BodyExt;
    use tower::util::ServiceExt;

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

    #[sqlx::test(migrations = "./migrations")]
    async fn test_get_user_by_id(pool: SqlitePool) {
        let app = app().with_state(pool);

        let req = http::Request::builder()
            .method(http::Method::POST)
            .uri("/user")
            .header("content-type", "application/json")
            .body(Body::from(
                serde_json::to_vec(&serde_json::json!({
                    "name": "TestUser",
                    "at_id": "test_user",
                    "birthday": "2023-01-01",
                }))
                .unwrap(),
            ))
            .unwrap();

        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::CREATED);

        let body = res.into_body().collect().await.unwrap().to_bytes();
        let user: User = serde_json::from_slice(&body).unwrap();
        assert_eq!(user.name, "TestUser");
        assert_eq!(user.at_id, "test_user");
        assert_eq!(user.birthday, Some("2023-01-01".to_string()));
    }
}
