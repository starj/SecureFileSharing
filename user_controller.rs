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

fn handle_register(data: &mut UserData, username: &str, password: &str) -> HttpResponse {
    if data.users.contains_key(username) {
        return conflict_response();
    } else {
        data.users.insert(username.to_string(), password.to_string());
        return success_register_response();
    }
}

fn handle_login(data: &mut UserData, username: &str, password: &str) -> HttpResponse {
    match data.users.get(username) {
        Some(user_password) if user_password == password => {
            let session_id = generate_session_id();
            data.sessions.insert(session_id.clone(), username.to_string());
            success_login_response(session_id)
        }
        _ => invalid_credentials_response(),
    }
}

fn conflict_response() -> HttpResponse {
    HttpResponse::Conflict().json(AuthResponse {
        session_id: None,
        message: "Username already exists".into(),
    })
}

fn success_register_response() -> HttpResponse {
    HttpResponse::Ok().json(AuthResponse {
        session_id: None,
        message: "User registered successfully".into(),
    })
}

fn success_login_response(session_id: String) -> HttpResponse {
    HttpResponse::Ok().json(AuthResponse {
        session_id: Some(session_id),
        message: "Login successful".into(),
    })
}

fn invalid_credentials_response() -> HttpResponse {
    HttpResponse::Unauthorized().json(AuthResponse {
        session_id: None,
        message: "Invalid username or password".into(),
    })
}

fn generate_session_id() -> String {
    "example_session_id".to_string()
}

#[post("/register")]
async fn register(body: web::Json<LoginRequest>, state: web::Data<AppState>) -> impl Responder {
    let mut data = state.lock().unwrap();
    handle_register(&mut data, &body.username, &body.password)
}

#[post("/login")]
async fn login(body: web::Json<LoginRequest>, state: web::Data<AppState>) -> impl Responder {
    let mut data = state.lock().unwrap();
    handle_login(&mut data, &body.username, &body.password)
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