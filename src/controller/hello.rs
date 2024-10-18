
use actix_web::{web, HttpResponse, Responder};
use crate::model::user::NewUser;
//use chrono::Utc;
use crate::db;
pub async fn index() -> impl Responder {
   let users=db::connection::get_users().unwrap();
    HttpResponse::Ok().json(users)
}   

pub async fn hello_name(name: web::Path<String>) -> impl Responder {
    let user=NewUser{
        name:&name,
        email:"deneme11",
        created_at: std::time::SystemTime::now()
    };
    let retval=db::connection::add_user(user);
    HttpResponse::Ok().body(format!("Hello, {}! retval= {:?}", name,retval))
}