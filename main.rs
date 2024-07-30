use actix_web::{web, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use std::env;

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello from Actix-web!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let server_address = env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind(&server_address)?
    .run()
    .await
}