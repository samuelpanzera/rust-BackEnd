mod db;
mod models;
mod repository;
mod schema;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use models::user::{NewUser, UpdateUser};
use repository::user_repository;

async fn create_user_handler(new_user: web::Json<NewUser>) -> impl Responder {
    let new_user = models::user::NewUser {
        email: new_user.email.clone(),
        name: new_user.name.clone(),
    };

    match user_repository::create_user(&new_user) {
        Ok(created_user) => HttpResponse::Ok().json(created_user),
        Err(_) => HttpResponse::InternalServerError().body("Failed to create user"),
    }
}

async fn read_user_handler(user_id: web::Path<i32>) -> impl Responder {
    match user_repository::read_user(user_id.into_inner()) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().body("Failed to read user"),
    }
}

async fn search_all_users_handler() -> impl Responder {
    match user_repository::search_all_users() {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().body("Failed to read users"),
    }
}

async fn update_user_handler(
    user_id: web::Path<i32>,
    updated_user: web::Json<UpdateUser>,
) -> impl Responder {
    let user = UpdateUser {
        name: updated_user.name.clone(),
        email: updated_user.email.clone(),
    };

    match user_repository::update_user(user_id.into_inner(), &user) {
        Ok(updated_user) => HttpResponse::Ok().json(updated_user),
        Err(_) => HttpResponse::InternalServerError().body("Failed to update user"),
    }
}

async fn delete_user_handler(user_id: web::Path<i32>) -> impl Responder {
    match user_repository::delete_user(user_id.into_inner()) {
        Ok(_) => HttpResponse::Ok().body("User deleted successfully"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to delete user"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/users", web::post().to(create_user_handler))
            .route("/users/{id}", web::get().to(read_user_handler))
            .route("/users/{id}", web::put().to(update_user_handler))
            .route("/users/{id}", web::delete().to(delete_user_handler))
            .route("/users", web::get().to(search_all_users_handler))
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}
