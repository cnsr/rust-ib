use crate::boards::*;
use actix_web::{get, post, delete, put, web, HttpResponse, Responder};
use sqlx::{Postgres, Pool, Error};

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(get_post_by_id);
    config.service(get_all_opposts);
    config.service(create_post);
}