use actix_web::{web::ServiceConfig, Scope};
use utoipa_swagger_ui::SwaggerUi;

pub fn configure(
    openapi: utoipa::openapi::OpenApi,
    route: Scope,
) -> impl FnOnce(&mut ServiceConfig) {
    move |cfg: &mut ServiceConfig| {
        cfg.service(route)
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi));
    }
}
