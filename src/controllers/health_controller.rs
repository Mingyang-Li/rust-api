use actix_web::Responder;

#[actix_web::get("/health")]
async fn health() -> impl Responder {
    format!("Server is running")
}
