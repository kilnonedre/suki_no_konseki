use actix_web::web::ServiceConfig;
use utoipa_swagger_ui::SwaggerUi;

use crate::route::build_api_route;

pub fn configure(openapi: utoipa::openapi::OpenApi) -> impl FnOnce(&mut ServiceConfig) {
    move |cfg: &mut ServiceConfig| {
        cfg.service(build_api_route())
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi));
    }
}
