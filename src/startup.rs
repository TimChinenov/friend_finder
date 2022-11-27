use crate::controllers::create_user;
use crate::controllers::get_user;
// use actix_web::middleware::Logger;
// use sqlx::PgPool;
use crate::controllers::health_check;
use std::net::TcpListener;
use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use sqlx::PgPool;

pub fn run(
    listener: TcpListener,
db_pool: PgPool) -> Result<Server, std::io::Error> {
        let db_pool = web::Data::new(db_pool);

        let server =HttpServer:: new(move || {
            App::new()
                .route("/health_check", web::get().to(health_check))
                .route("/user", web::post().to(create_user))
                .route("/user", web::get().to(get_user))
                .app_data(db_pool.clone())
        })
        .listen(listener)?
        .run();

        Ok(server)
}