use actix_web::{HttpResponse, Responder};


pub async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body("404 Not Found")
}