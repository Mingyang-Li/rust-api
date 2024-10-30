use actix_web::{web, HttpResponse};
use serde::Serialize;
use utoipa::ToSchema;
use validator::{Validate, ValidationErrors};
use validator_derive::Validate;
use std::collections::HashMap;

#[derive(Serialize, ToSchema)]
pub struct User {
    #[schema(example = "abc", required = false)]
    id: String,

    #[schema(example = "Nick", required = false)]
    first_name: String,

    #[schema(example = "Anderson", required = false)]
    last_name: String,
}

#[derive(serde::Deserialize, ToSchema, Validate)]
pub struct UserCreateInput {
    #[schema(example = "Nick", required = true)]
    #[validate(length(min = 1, message = "Must contain at least 1 character"))]
    first_name: String,

    #[schema(example = "Anderson", required = true)]
    #[validate(length(min = 1, message = "Must contain at least 1 character"))]
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
    if let Err(errors) = input.validate() {
        return HttpResponse::BadRequest().json(format_validation_errors(&errors));
    }

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

#[derive(Debug, Serialize)]
struct ErrorResponse {
    errors: Vec<HashMap<String, String>>,
}

fn format_validation_errors(errors: &ValidationErrors) -> ErrorResponse {
    let mut error_list = Vec::new();

    for (field, field_errors) in errors.field_errors() {
        for error in field_errors {
            let mut error_obj = HashMap::new();
            // Insert the field name and the actual error message from the validator
            let message = error
                .message
                .clone()
                .unwrap_or_else(|| "validation error".into())
                .to_string();
            error_obj.insert(field.to_string(), message);
            error_list.push(error_obj);
        }
    }

    ErrorResponse { errors: error_list }
}