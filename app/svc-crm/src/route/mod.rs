use actix_web::{web, Scope};

use crate::route::users::build_user_route;

pub mod users;

pub fn build_api_route() -> Scope {
    web::scope("/api").service(build_user_route())
}
