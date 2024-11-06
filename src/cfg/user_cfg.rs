use actix_web::web;
use crate::controller::user_controller::{get_all_users, get_user_by_id, create_user, update_user, delete_user, get_user_by_email, get_user_by_email_and_password};

pub fn set_user_cfg(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/user")
            .route(web::get().to(get_all_users))
    );
    cfg.service(
        web::resource("/user/{id}")
            .route(web::get().to(get_user_by_id))
    );
    cfg.service(
        web::resource("/user/create")
            .route(web::post().to(create_user))
    );
    cfg.service(
        web::resource("/user/update/{id}")
            .route(web::put().to(update_user))
    );
    cfg.service(
        web::resource("/user/delete/{id}")
            .route(web::delete().to(delete_user))
    );
    cfg.service(
        web::resource("/user/get_user_by_email/{email}")
            .route(web::get().to(get_user_by_email))
    );
    cfg.service(
        web::resource("/user/get_user_by_email_and_password/{email}/{password}")
            .route(web::get().to(get_user_by_email_and_password))
    );
   
}