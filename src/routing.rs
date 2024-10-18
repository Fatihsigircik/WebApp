use actix_web::web;
use crate::controller::hello::{index, hello_name};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::get().to(index))
    );
    cfg.service(
        web::resource("/hello/{name}")
            .route(web::get().to(hello_name))
    );
}