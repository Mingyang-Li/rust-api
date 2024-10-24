use actix_web::{web, HttpResponse};
use serde::Serialize;

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
