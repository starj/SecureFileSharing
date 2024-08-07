use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use std::env;

#[post("/upload")]
async fn upload(file: web::Json<FileDetails>) -> impl Responder {
    HttpResponse::Ok().body(format!("File '{}' uploaded successfully!", file.name))
}

#[post("/download")]
async fn download(file: web::Json<FetchRequest>) -> impl Responder {
    HttpResponse::Ok().body(format!("File '{}' ready for download!", file.name))
}

#[post("/share")]
async fn share(file: web::Json<ShareDetails>) -> impl Responder {
    HttpResponse::Ok().body(format!("File '{}' shared successfully! Access using: {}", file.name, file.url))
}

#[derive(serde::Serialize, serde::Deserialize)]
struct FileDetails {
    name: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct FetchRequest {
    name: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct ShareDetails {
    name: String,
    url: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let server = HttpServer::new(|| {
        App::new()
            .service(upload)
            .service(download)
            .service(share)
    });

    let server_address = env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    println!("Server running at http://{}", server_address);

    server.bind(server_address)?.run().await
}