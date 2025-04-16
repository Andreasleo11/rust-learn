use axum::{Json, extract::State, response::IntoResponse};
use bcrypt::{DEFAULT_COST, hash};
use serde::Deserialize;
use sqlx::MySqlPool;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct RegisterUser {
    pub name: String,
    pub password: String,
}

pub async fn register_user(
    State(pool): State<Arc<MySqlPool>>,
    Json(payload): Json<RegisterUser>,
) -> impl IntoResponse {
    let hashed_password = hash(payload.password, DEFAULT_COST).unwrap();

    let user_id = Uuid::new_v4();
    let query = sqlx::query!(
        "INSERT INTO users (id, name, password) VALUES (?, ?, ?)",
        user_id.to_string(),
        payload.name,
        hashed_password
    )
    .execute(pool.as_ref())
    .await;

    match query {
        Ok(_) => (
            axum::http::StatusCode::CREATED,
            Json("User registered successfully"),
        ),
        Err(_) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json("Failed to register user"),
        ),
    }
}
