use crate::auth::middleware::AuthToken;
use axum::{Router, routing::get};
use sqlx::MySqlPool;
use std::sync::Arc;

async fn protected_handler(AuthToken(claims): AuthToken) -> String {
    format!("Hai {}, lo berhasil akses endpoint rahasia", claims.sub)
}

pub fn protected_routes() -> Router<Arc<MySqlPool>> {
    Router::new().route("/protected", get(protected_handler))
}
