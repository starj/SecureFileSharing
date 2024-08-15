use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use actix_web::{post, web, App, HttpServer, HttpResponse, Responder};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};

struct UserData {
    sessions: HashMap<String, String>,
    users: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct AuthResponse {
    session_id: Option<String>,
    message: String,
}

type AppState = Arc<Mutex<UserData>>;

#[post("/register")]
async fn register(body: web::Json<LoginRequest>, state: web::Data<AppState>) -> impl Responder {
    let mut data = state.lock().unwrap();

    if data.users.contains_key(&body.username) {
        HttpResponse::Conflict().json(AuthResponse {
            session_id: None,
            message: "Username already exists".into(),
        })
    } else {
        data.users.insert(body.username.clone(), body.password.clone());
        HttpResponse::Ok().json(AuthResponse {
            session_id: None,
            message: "User registered successfully".into(),
        })
    }
}

#[post("/login")]
async fn login(body: web::Json<LoginRequest>, state: web::Data<AppState>) -> impl Responder {
    let mut data = state.lock().unwrap();

    match data.users.get(&body.username) {
        Some(password) if password == &body.password => {
            let session_id = "example_session_id".to_string();
            data.sessions.insert(session_id.clone(), body.username.clone());
            HttpResponse::Ok().json(AuthResponse {
                session_id: Some(session_id),
                message: "Login successful".into(),
            })
        }
        _ => HttpResponse::Unauthorized().json(AuthResponse {
            session_id: None,
            message: "Invalid username or password".into(),
        }),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); 

    let user_data = UserData {
        sessions: HashMap::new(),
        users: HashMap::new(),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Arc::new(Mutex::new(user_data.clone()))))
            .service(register)
            .service(login)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}