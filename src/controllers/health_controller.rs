use actix_web::Responder;

#[utoipa::path(
    get,
    path = "/user{id}",
    params(
        ("id" = String, Path, description = "User ID"),
    ),
    responses(
        (status = 200, description = "Server is running"),
        (status = 500, description = "Internal server error")
    )
)]
#[actix_web::get("/health")]
async fn health() -> impl Responder {
    format!("Server is running")
}
