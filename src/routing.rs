use actix_web::web;
use crate::cfg::user_cfg;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::get().to(|| async { "{\"developer\" : \"Fatih SIĞIRCIK\", \"year\" : \"2024\"} " })));
    user_cfg::set_user_cfg(cfg); // adding endpoints for user
}