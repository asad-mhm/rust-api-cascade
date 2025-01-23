// Main function

mod routes;
use crate::routes::handlers::health;
use crate::routes::handlers::callback_get;
use crate::routes::handlers::callback_post;
use crate::routes::handlers::callback_delete;


use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(health)
            .service(callback_get)
            .service(callback_post)
            .service(callback_delete)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}