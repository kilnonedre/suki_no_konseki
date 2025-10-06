use actix_web::{web, Scope};

use crate::controller::auth_controller::login;

pub fn build_auth_route() -> Scope {
    web::scope("").route("/login", web::post().to(login))
}
