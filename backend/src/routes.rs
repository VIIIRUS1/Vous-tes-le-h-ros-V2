use axum::{Json, response::IntoResponse};
use serde::Serialize;
use sqlx::Row;
use crate::db::get_pool;

#[derive(Serialize)]
struct User {
    id: i32,
    name: String,
}

pub async fn get_users() -> impl IntoResponse {
    let pool = get_pool();
    let rows = sqlx::query("SELECT id, name FROM users")
        .fetch_all(pool)
        .await
        .unwrap();

    let users: Vec<User> = rows.iter().map(|row| User {
        id: row.get("id"),
        name: row.get("name"),
    }).collect();

    Json(users)
}
