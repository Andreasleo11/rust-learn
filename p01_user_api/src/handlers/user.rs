use crate::models::user::User;
use crate::models::user::UserInput;
use actix_web::{HttpResponse, web};
use sqlx::MySqlPool;
use std::sync::Arc;

use argon2::{Argon2, PasswordHasher};
use password_hash::SaltString;
use rand_core::OsRng;
// Function to hash a password

async fn hash_password(password: &str) -> Option<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    match argon2.hash_password(password.as_bytes(), &salt) {
        Ok(hash) => Some(hash.to_string()),
        Err(_) => None,
    }
}

// Handler untuk route /users

// Handler untuk route /users
pub async fn get_users(pool: web::Data<Arc<MySqlPool>>) -> HttpResponse {
    // Mendapatkan referensi ke MySqlPool
    let pool_ref: &MySqlPool = &**pool;

    // Mengambil data dari database
    let users = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(pool_ref) // Gunakan pool_ref sebagai referensi ke MySqlPool
        .await
        .expect("Failed to fetch users");

    // Mengembalikan data dalam format JSON
    HttpResponse::Ok().json(users)
}

pub async fn create_user(
    pool: web::Data<Arc<MySqlPool>>,
    new_user: web::Json<UserInput>,
) -> HttpResponse {
    let user = &new_user;

    // 🔒 Hash password-nya dulu
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let hashed_password = match argon2.hash_password(user.password.as_bytes(), &salt) {
        Ok(hash) => hash.to_string(),
        Err(_) => return HttpResponse::InternalServerError().body("Failed to hash password"),
    };

    let result = sqlx::query!(
        r#"
        INSERT INTO users (name, email, password, role, created_at)
        VALUES (?, ?, ?, ?, ?)
        "#,
        user.name,
        user.email,
        hashed_password,
        user.role.as_deref(),
        chrono::Utc::now().naive_utc()
    )
    .execute(pool.as_ref().as_ref()) // 👈 Solusi penting di sini
    .await;

    match result {
        Ok(_) => HttpResponse::Created().json("User created successfully"),
        Err(e) => {
            eprintln!("DB Error: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to create user")
        }
    }
}

pub async fn update_user(
    pool: web::Data<Arc<MySqlPool>>,
    user_id: web::Path<i32>,
    updated_user: web::Json<UserInput>, // Bisa menggunakan UserInput supaya tidak mengupdate id dan created_at
) -> HttpResponse {
    // Hash password jika diubah
    let hashed_password = if !updated_user.password.is_empty() {
        Some(hash_password(&updated_user.password).await)
    } else {
        None
    };

    // Update data user berdasarkan ID
    let result = match hashed_password {
        Some(password) => {
            // Jika password baru diberikan, update dengan password yang baru
            sqlx::query!(
                r#"
                UPDATE users 
                SET name = ?, email = ?, password = ?, role = ?
                WHERE id = ?
                "#,
                updated_user.name,
                updated_user.email,
                password, // Gunakan password yang sudah di-hash
                updated_user.role.as_deref(),
                user_id.into_inner()
            )
            .execute(pool.as_ref().as_ref())
            .await
        }
        None => {
            // Jika password tidak diubah, update tanpa password
            sqlx::query!(
                r#"
                UPDATE users 
                SET name = ?, email = ?, role = ?
                WHERE id = ?
                "#,
                updated_user.name,
                updated_user.email,
                updated_user.role.as_deref(),
                user_id.into_inner()
            )
            .execute(pool.as_ref().as_ref())
            .await
        }
    };

    match result {
        Ok(_) => HttpResponse::Ok().json(updated_user),
        Err(_) => HttpResponse::InternalServerError().body("Failed to update user"),
    }
}

pub async fn delete_user(pool: web::Data<Arc<MySqlPool>>, user_id: web::Path<i32>) -> HttpResponse {
    // Menghapus user berdasarkan ID
    let result = sqlx::query!(
        r#"
        DELETE FROM users WHERE id = ?
        "#,
        user_id.into_inner()
    )
    .execute(pool.as_ref().as_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::NoContent().finish(), // Tidak ada konten yang dikembalikan
        Err(_) => HttpResponse::InternalServerError().body("Failed to delete user"),
    }
}
