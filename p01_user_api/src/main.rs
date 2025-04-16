mod config;
mod handlers;
mod models;
mod routes;

use crate::config::establish_connection;
use actix_web::{App, HttpServer};
use env_logger;
use std::sync::Arc;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Koneksi ke database
    env_logger::init();
    let pool = Arc::new(establish_connection().await);

    // Logging sederhana
    println!("🚀 Server jalan di http://127.0.0.1:8000");

    // Jalankan HTTP server
    HttpServer::new(move || App::new().service(routes::create_routes(pool.clone())))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
