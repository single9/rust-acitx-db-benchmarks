mod controller;
mod service;

use actix_web::web;

pub fn init() -> actix_web::Scope {
    web::scope("/calculator")
        .service(controller::test2)
        .service(controller::test3)
}
