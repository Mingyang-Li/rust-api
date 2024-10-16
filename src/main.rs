use actix_web::{App, HttpServer, Responder};

#[actix_web::get("/health")]
async fn health() -> impl Responder {
  format!("Server is running")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    println!("Server_is_running_on_port_{port}");

    HttpServer::new(|| {
        App::new()
        .service(health)
    })
    .bind(("127.0.0.1", port))?
    .workers(2)
    .run()
    .await
}