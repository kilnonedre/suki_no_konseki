use actix_web::{web, Scope};

pub fn build_v1_route() -> Scope {
    web::scope("/v1")
}
