use actix_web::{web, Scope};

pub mod users_route;

pub fn build_v1_route() -> Scope {
    web::scope("/v1").service(users_route::build_user_route())
}
