use std::collections::HashMap;

use axum::extract::State;
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use sqlx::postgres::PgQueryResult;

use crate::models::users::User;
use crate::models::users::{self, CreateUser};
use crate::AppState;

// pub async fn create_user(
//     // this argument tells axum to parse the request body
//     // as JSON into a `CreateUser` type
//     Json(payload): Json<CreateUser>,
// ) -> impl IntoResponse {
//     // insert your application logic here
//     let user = User {
//         id: 1337,
//         username: "Hos".to_string(),
//     };

//     // this will be converted into a JSON response
//     // with a status code of `201 Created`
//     (StatusCode::CREATED, Json(user))
// }

pub async fn all_users(State(state): State<AppState>) -> Json<Vec<User>> {
    let rows = sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(&state.pool)
        .await
        .expect("err getting users");

    Json(rows)
}

pub async fn get_user(State(state): State<AppState>, Path(id): Path<i32>) -> Json<User> {
    let row = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
        .fetch_one(&state.pool)
        .await
        .expect("err getting user");

    Json(row)
}

pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<CreateUser>,
) -> Json<serde_json::Value> {
    let name = payload.name;
    let query_result = sqlx::query("UPDATE users SET name = $2 WHERE id = $1")
        .bind(id)
        .bind(name)
        .execute(&state.pool)
        .await;

    match query_result {
        Ok(result) => {
            if result.rows_affected() == 0 {
                let message = format!("Note with ID: {} not found", id);
                Json(json!({"status": "fail","message": message}))
            } else {
                let message = format!("Success updating {}", id);
                Json(json!({"status": "error","message": message}))
            }
        }
        Err(e) => {
            let message = format!("Internal server error: {}", e);
            Json(json!({"status": "error","message": message}))
        }
    }
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Json<serde_json::Value> {
    let name = payload.name;
    let query_result = sqlx::query("INSERT INTO users (name) VALUES ($1)")
        .bind(name)
        .execute(&state.pool)
        .await;

    match query_result {
        Ok(result) => {
            if result.rows_affected() == 0 {
                let message = format!("Note with ID: not found");
                Json(json!({"status": "fail","message": message}))
            } else {
                let message = format!("Success creating ");
                Json(json!({"status": "error","message": message}))
            }
        }
        Err(e) => {
            let message = format!("Internal server error: {}", e);
            Json(json!({"status": "error","message": message}))
        }
    }
}

pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Json<serde_json::Value> {
    let query_result = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await;

    match query_result {
        Ok(result) => {
            if result.rows_affected() == 0 {
                let message = format!("Note with ID: {} not found", id);
                Json(json!({"status": "fail","message": message}))
            } else {
                let message = format!("Success deleting {}", id);
                Json(json!({"status": "error","message": message}))
            }
        }
        Err(e) => {
            let message = format!("Internal server error: {}", e);
            Json(json!({"status": "error","message": message}))
        }
    }
}
