
use actix_web::{web, HttpResponse, Responder};
use crate::model::{user::User, user::NewUser};
use crate::db;
pub async fn index() -> impl Responder {
    let user = User{
        id:12,
        name:"John".to_string(),
        email:"Doe".to_string(),
        password:"1234567890".to_string()
    };
    let user=user;
    HttpResponse::Ok().json(user)
}   

pub async fn hello_name(name: web::Path<String>) -> impl Responder {
    let user=NewUser{
        name:&name,
        email:"deneme"
    };
    let retval=db::connection::add_user(user);
    HttpResponse::Ok().body(format!("Hello, {}! retval= {:?}", name,retval))
}