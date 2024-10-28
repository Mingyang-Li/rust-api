use actix_web::{web, HttpResponse};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct User {
    #[schema(example = "abc", required = false)]
    id: String,

    #[schema(example = "Nick", required = false)]
    first_name: String,

    #[schema(example = "Anderson", required = false)]
    last_name: String,
}

#[derive(serde::Deserialize, ToSchema)]
pub struct UserCreateInput {
    #[schema(example = "Nick", required = true)]
    first_name: String,

    #[schema(example = "Anderson", required = true)]
    last_name: String,
}

#[utoipa::path(
    post,
    path = "/user",
    request_body = UserCreateInput,
    responses(
        (status = 201, description = "User created", body = User),
        (status = 500, description = "Internal server error")
    )
)]
#[actix_web::post("/user")]
async fn create_user(input: web::Json<UserCreateInput>) -> HttpResponse {
    HttpResponse::Created().json(User {
        id: "AUTO_GENERATED_ID".to_string(),
        first_name: input.first_name.to_string(),
        last_name: input.last_name.to_string(),
    })
}

#[utoipa::path(
    get,
    path = "/user/{id}",
    params(
        ("id" = String, description = "User ID"),
    ),
    responses(
        (status = 200, description = "User found", body = User),
        (status = 500, description = "Internal server error")
    )
)]
#[actix_web::get("/user/{id}")]
async fn find_one_user(id: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().json(User {
        id: id.to_string(),
        first_name: "Nick".to_string(),
        last_name: "Anderson".to_string(),
    })
}
