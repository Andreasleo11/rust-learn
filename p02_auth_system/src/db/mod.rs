use sqlx::{MySql, MySqlPool, Pool};
use std::env;

pub async fn create_pool() -> Result<MySqlPool, sqlx::Error> {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

    let pool = MySqlPool::connect(&database_url).await?;

    Ok(pool)
}
