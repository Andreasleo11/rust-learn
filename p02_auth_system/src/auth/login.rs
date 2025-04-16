use crate::auth::jwt::create_jwt;
use axum::{Json, extract::State, response::IntoResponse};
use bcrypt::verify;
use serde::Deserialize;
use sqlx::MySqlPool;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct LoginUser {
    pub name: String,
    pub password: String,
}

pub async fn login_user(
    State(pool): State<Arc<MySqlPool>>,
    Json(payload): Json<LoginUser>,
) -> impl IntoResponse {
    let row = sqlx::query!(
        "SELECT name,password FROM users WHERE name = ?",
        payload.name
    )
    .fetch_one(pool.as_ref())
    .await;

    match row {
        Ok(user) => {
            if verify(&payload.password, &user.password).unwrap_or(false) {
                // Generate JWT after successful login
                let token = create_jwt(&user.name.to_string()).unwrap();
                Json(token)
            } else {
                Json("Invalid credentials".to_string())
            }
        }
        Err(_) => Json("User not found".to_string()),
    }
}
