use actix_web::{web, Scope};

use crate::route::v1::auth_route::build_auth_route;

pub mod auth_route;

pub fn build_v1_route() -> Scope {
    web::scope("/v1").service(build_auth_route())
}
