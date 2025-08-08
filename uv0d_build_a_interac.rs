use actix_web::{web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Define a struct to hold API data
#[derive(Serialize, Deserialize)]
struct ApiResponse {
    message: String,
    data: HashMap<String, String>,
}

async fn api_handler(req_path: web::Path<String>) -> impl Responder {
    let api_response = ApiResponse {
        message: "Welcome to interactive API service!".to_string(),
        data: HashMap::from([("path".to_string(), req_path.to_string())]),
    };
    web::Json(api_response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/api/{path}").route(web::get().to(api_handler)))
            .service(web::resource("/api/").route(web::get().to(api_handler)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}