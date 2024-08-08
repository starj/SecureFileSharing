use actix_web::{post, web, App, HttpResponse, HttpServer, Responder, http::StatusCode, error::ResponseError};
use actix_web::dev::HttpResponseBuilder;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::{env, fmt};

#[post("/upload")]
async fn upload(file: web::Json<FileDetails>) -> Result<HttpResponse, ServiceError> {
    Ok(HttpResponse::Ok().body(format!("File '{}' uploaded successfully!", file.name)))
}

#[post("/download")]
async fn download(file: web::Json<FetchRequest>) -> Result<HttpResponse, ServiceError> {
    Ok(HttpResponse::Ok().body(format!("File '{}' ready for download!", file.name)))
}

#[post("/share")]
async fn share(file: web::Json<ShareDetails>) -> Result<HttpResponse, ServiceError> {
    Ok(HttpResponse::Ok().body(format!("File '{}' shared successfully! Access using: {}", file.name, file.url)))
}

#[derive(Debug)]
struct ServiceError {
    message: String,
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl ResponseError for ServiceError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code()).body(self.message.clone())
    }
}

#[derive(Serialize, Deserialize)]
struct FileDetails {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct FetchRequest {
    name: String,
}

#[derive(Serialize, Deserialize)]
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