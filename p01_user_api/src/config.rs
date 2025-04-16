use dotenv::dotenv;
use sqlx::MySqlPool;
use std::env;

pub async fn establish_connection() -> MySqlPool {
    dotenv().ok(); // Load .env file

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

    // Membuat koneksi ke database MySQL
    MySqlPool::connect(&database_url)
        .await
        .expect("Failed to create MySQL pool")
}
