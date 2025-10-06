use actix_web::{web, Scope};

use crate::route::v1::build_v1_route;

pub mod v1;

pub fn build_api_route() -> Scope {
    web::scope("/api").service(build_v1_route())
}
