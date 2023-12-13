use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[post("/add_product")]
pub async fn add_product(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
