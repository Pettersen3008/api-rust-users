use actix_web::web;
use actix_web::{web::{
    Data,
    Json,
}, get, post, put, delete, HttpResponse};
use crate::{
    models::users::User,
    repository::database::Database,
};

#[get("/users")]
pub async fn get_users(db: Data<Database>) -> HttpResponse {
    let users = db.get_users();
    match users {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/users")]
pub async fn create_user(db: Data<Database>, user: Json<User>) -> HttpResponse {
    let user = db.create_user(user.into_inner());
    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/users/{user_id}")]
pub async fn get_user_by_id(db: Data<Database>, user_id: web::Path<String>) -> HttpResponse {
    let user = db.get_user_by_id(&user_id);
    match user {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().finish(),
    }
}

#[put("/users/{user_id}")]
pub async fn update_user(db: Data<Database>, user_id: web::Path<String>, user: Json<User>) -> HttpResponse {
    let user = db.update_user(&user_id, user.into_inner());
    match user {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().finish(),
    }
}

#[delete("/users/{user_id}")]
pub async fn delete_user(db: Data<Database>, user_id: web::Path<String>) -> HttpResponse {
    let user = db.delete_user(&user_id);
    match user {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().finish(),
    }
}


pub fn config(cfn: &mut web::ServiceConfig) {
    cfn.service(web::scope("/api")
        .service(get_users)
        .service(create_user)
        .service(get_user_by_id)
        .service(update_user)
        .service(delete_user)
    );
}