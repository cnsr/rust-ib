use crate::admin::*;

use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::{Postgres, Pool};

// ADMIN apps in the 'admin' scope don't need to have full path specified

//unprotected route - needs full path
#[post("/admin/login/")]
async fn admin_login() -> impl Responder {
    HttpResponse::Ok()
}

#[post("/verify/")]
async fn admin_verify() -> impl Responder {
    HttpResponse::Ok()
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(admin_verify);
}

pub fn init_unprotected_routes(config: &mut web::ServiceConfig) {
    config.service(admin_login);
}