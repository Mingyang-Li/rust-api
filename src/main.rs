use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use std::env;

#[actix_web::get("/health")]
async fn health() -> impl Responder {
    format!("Server is running")
}

#[derive(Serialize)]
struct User {
    id: String,
    first_name: String,
    last_name: String,
}

#[derive(serde::Deserialize)]
struct UserCreateInput {
    first_name: String,
    last_name: String,
}

#[actix_web::post("/user")]
async fn create_user(input: web::Json<UserCreateInput>) -> HttpResponse {
    HttpResponse::Created().json(User {
        id: "AUTO_GENERATED_ID".to_string(),
        first_name: input.first_name.to_string(),
        last_name: input.last_name.to_string(),
    })
}

#[actix_web::get("/user/{id}")]
async fn find_one_user(id: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().json(User {
        id: id.to_string(),
        first_name: "Nick".to_string(),
        last_name: "Anderson".to_string(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = env::var("PORT")
    .unwrap_or_else(|_| "8080".to_string())
    .parse()
    .expect("PORT must be a number");

    println!("Server is running on port {port}");

    HttpServer::new(|| {
        App::new()
            .service(health)
            .service(find_one_user)
            .service(create_user)
    })
    .bind(("0.0.0.0", port))?
    .workers(2)
    .run()
    .await
}
