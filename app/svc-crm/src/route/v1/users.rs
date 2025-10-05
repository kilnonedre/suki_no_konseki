use actix_web::{web, Scope};

use crate::controller::user_controller::{create, delete, list, read, update};

pub fn build_user_route() -> Scope {
    web::scope("/users")
        .route("", web::post().to(create))
        .route("/{id}", web::get().to(read))
        .route("/{id}", web::put().to(update))
        .route("/{id}", web::delete().to(delete))
        .route("", web::get().to(list))
}
