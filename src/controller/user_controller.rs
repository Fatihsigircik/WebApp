

use actix_web::{web, HttpResponse, Responder};

use crate::model::user::NewUser;
use crate::services::user_service;

pub async fn create_user(user: web::Json<NewUser>) -> impl Responder {    
   let user = user.into_inner(); 
   match  user_service::add_user(user){
    Ok(created_user) => HttpResponse::Ok().json(created_user), // Başarılı sonuç JSON olarak döner
    Err(_) => HttpResponse::InternalServerError().body("Kullanıcı oluşturulamadı"),
   }
}

pub async fn get_user_by_id(id: web::Path<i32>) -> impl Responder {
    match user_service::get_user_by_id(id.into_inner()).await{
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().body("Kullanıcı bulunamadı")
    }
}
pub async fn get_all_users() -> impl Responder {
    match user_service::get_all_users().await{
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().body("Kullanıcılar bulunamadı")
    }
}

pub async fn update_user(id: web::Path<i32>, user: web::Json<NewUser>) -> impl Responder {
    match user_service::update_user(id.into_inner(), user.into_inner()).await{
        Ok(updated_user) => HttpResponse::Ok().json(updated_user),
        Err(_) => HttpResponse::InternalServerError().body("Kullanıcı güncellenemedi")
    }
}

pub async fn delete_user(id: web::Path<i32>) -> impl Responder {
    match user_service::delete_user(id.into_inner()).await{
        Ok(_) => HttpResponse::Ok().body("Kullanıcı silindi"),
        Err(_) => HttpResponse::InternalServerError().body("Kullanıcı silinemedi")
    }
}

pub async fn get_user_by_email(email: web::Path<String>) -> impl Responder {
    match user_service::get_user_by_email(email.into_inner()).await{
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().body("Kullanıcı bulunamadı")
    }
}

pub async fn get_user_by_email_and_password(email: web::Path<(String, String)>) -> impl Responder {
    match user_service::get_user_by_email_and_password(email.0.clone(), email.1.clone()).await{
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().body("Kullanıcı bulunamadı")
    }
}



