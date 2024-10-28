use actix_web::{App, HttpServer};
use std::env;
mod controllers;
use controllers::{health_controller, user_controller};
use serde::{Deserialize, Serialize};
use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi, ToSchema,
};
use utoipa_swagger_ui::SwaggerUi;

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct TokenClaims {
    id: i32,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    println!("Server is running on port {port}");

    #[derive(OpenApi)]
    #[openapi(
        paths(
            health_controller::health,
            user_controller::find_one_user,
            user_controller::create_user,
        ),
        components(schemas(TokenClaims, user_controller::User, user_controller::UserCreateInput)),
        modifiers(&SecurityAddon)
    )]

    struct ApiDoc;

    struct SecurityAddon;
    impl Modify for SecurityAddon {
        fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
            let components = openapi.components.as_mut().unwrap();
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
            components.add_security_scheme(
                "basic_auth",
                SecurityScheme::Http(HttpBuilder::new().scheme(HttpAuthScheme::Basic).build()),
            )
        }
    }

    let openapi = ApiDoc::openapi();

    HttpServer::new(move|| {
        App::new()
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .service(health_controller::health)
            .service(user_controller::find_one_user)
            .service(user_controller::create_user)
    })
    .bind(("0.0.0.0", port))?
    .workers(2)
    .run()
    .await
}
