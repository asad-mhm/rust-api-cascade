use actix_web::{delete, get, http::header::ContentType, post, web, HttpResponse, Responder};
use mt_logger::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Callback {
    endpoint: String,
    method: String,
    duration: u32,
}

#[get("/health")]
async fn health() -> impl Responder {
    mt_log!(Level::Info, "Request recieved at /health endpoint");
    let response: HttpResponse = HttpResponse::Ok().body("Healthy");
    response
}

#[get("/callback")]
async fn callback_get() -> impl Responder {
    HttpResponse::Ok().body("GET")
}

#[post("/callback")]
async fn callback_post(callback: web::Json<Callback>) -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body("recieved")
}

#[delete("/callback")]
async fn callback_delete() -> impl Responder {
    HttpResponse::Ok()
}
