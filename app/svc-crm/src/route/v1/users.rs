use actix_web::{web, Scope};

use crate::controller::user_controller::create_user;

pub fn build_user_route() -> Scope {
    web::scope("/users").route("", web::post().to(create_user))
}
