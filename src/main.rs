use actix_web::{App, HttpServer};
use std::env;
mod controllers;
use controllers::{user_controller, health_controller};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = env::var("PORT")
    .unwrap_or_else(|_| "8080".to_string())
    .parse()
    .expect("PORT must be a number");

    println!("Server is running on port {port}");

    HttpServer::new(|| {
        App::new()
            .service(health_controller::health)
            .service(user_controller::find_one_user)
            .service(user_controller::create_user)
    })
    .bind(("0.0.0.0", port))?
    .workers(2)
    .run()
    .await
}
