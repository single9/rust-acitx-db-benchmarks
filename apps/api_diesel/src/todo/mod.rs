mod controller;
mod service;

use actix_web::web;

pub fn init() -> actix_web::Scope {
    web::scope("/todo")
        .service(controller::list)
        .service(controller::create)
        .service(controller::get_todo)
}
