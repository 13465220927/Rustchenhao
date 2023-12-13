use actix_web::{get, post, web, App, HttpServer, Responder};
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(routes::add_product)
        // .service(echo)
        // .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 18001))?
    .run()
    .await
}
