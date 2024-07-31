[dependencies]
actix-web = "4.0"
dotenv = "0.15.0"
lazy_static = "1.4.0"
tokio = { version = "1", features = ["full"] }
```

```rust
use actix_web::{web, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use std::env;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref INDEX_CACHE: Mutex<Option<String>> = Mutex::new(None);
}

async fn index() -> HttpResponse {
    let mut cache = INDEX_CACHE.lock().unwrap();
    if let Some(cached_value) = &*cache {
        HttpResponse::Ok().body(cached_value.clone())
    } else {
        let response_body = "Hello from Actix-web!".to_string();
        *cache = Some(response_body.clone());
        HttpResponse::Ok().body(response_body)
    }
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