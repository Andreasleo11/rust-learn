use crate::handlers::user::{create_user, delete_user, get_users, update_user};
use actix_web::{Scope, web};
use sqlx::MySqlPool;
use std::sync::Arc;

pub fn create_routes(pool: Arc<MySqlPool>) -> Scope {
    web::scope("") // Kosong berarti root
        .app_data(web::Data::new(pool))
        .route("/get-users", web::get().to(get_users)) // GET
        .route("/users/create", web::post().to(create_user)) // POST
        .route("/users/update/{id}", web::put().to(update_user)) // PUT
        .route("/users/delete/{id}", web::delete().to(delete_user)) // DELETE
}
