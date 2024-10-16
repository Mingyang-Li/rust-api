use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

#[actix_web::get("/health")]
async fn health() -> impl Responder {
    format!("Server is running")
}

#[derive(Serialize)]
struct User {
    id: String,
    first_name: String,
}

#[actix_web::get("/user/{id}")]
async fn find_one_user(id: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().json(User {
        id: id.to_string(),
        first_name: "Nick".to_string(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    println!("Server is running on port {port}");

    HttpServer::new(|| {
        App::new()
            .service(health)
            .service(find_one_user)
    })
    .bind(("127.0.0.1", port))?
    .workers(2)
    .run()
    .await
}
