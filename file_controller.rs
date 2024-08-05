use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use std::env;

#[post("/upload")]
async fn upload_file(file: web::Json<FileUpload>) -> impl Responder {
    HttpResponse::Ok().body(format!("File {} uploaded successfully!", file.filename))
}

#[post("/download")]
async fn download_file(file: web::Json<FileRequest>) -> impl Responder {
    HttpResponse::Ok().body(format!("File {} downloaded successfully!", file.filename))
}

#[post("/share")]
async fn share_file(file: web::Json<FileShare>) -> impl Responder {
    HttpResponse::Ok().body(format!("File {} shared successfully! Link: {}", file.filename, file.link))
}

#[derive(serde::Serialize, serde::Deserialize)]
struct FileUpload {
    filename: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct FileRequest {
    filename: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct FileShare {
    filename: String,
    link: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let server = HttpServer::new(|| {
        App::new()
            .service(upload_file)
            .service(download_file)
            .service(share_file)
    });

    let server_address = env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    println!("Running server at http://{}", server_address);

    server.bind(server_address)?.run().await
}