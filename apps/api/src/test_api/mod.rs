mod controller;
mod service;

use actix_web::web;

pub fn init() -> actix_web::Scope {
    web::scope("/test").service(controller::test_name)
}
