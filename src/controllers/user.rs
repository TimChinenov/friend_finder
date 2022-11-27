use actix_web::{web, HttpResponse};
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct UserData {
    username: String,
    first_name: String,
    last_name: String,
    email: String
}

pub async fn create_user(
    form: web::Form<UserData>,
    pool: web::Data<PgPool>) -> HttpResponse {
        HttpResponse::Ok().finish()
}

pub async fn get_user() -> HttpResponse {
    HttpResponse::Ok().finish()
}