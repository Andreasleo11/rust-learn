mod auth;
mod db;
mod routes;

use crate::auth::{login::login_user, register::register_user};
use crate::routes::protected::protected_routes; // <- ini juga penting
use axum::Server;
use axum::{Router, routing::post};
use std::{net::SocketAddr, sync::Arc};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let pool = Arc::new(db::create_pool().await.unwrap());

    let app = Router::new()
        .route("/register", post(register_user))
        .route("/login", post(login_user))
        .merge(protected_routes())
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
