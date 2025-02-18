mod routes;
use crate::routes::handlers::callback_delete;
use crate::routes::handlers::callback_get;
use crate::routes::handlers::callback_post;
use crate::routes::handlers::health;

use actix_web::{App, HttpServer};
use mt_logger::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    mt_new!(None, Level::Info, OutputStream::StdOut);
    mt_level!(Level::Trace);
    mt_log!(Level::Info, "Starting server.");
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
