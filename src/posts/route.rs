use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json::json;
use sqlx::{Postgres, Pool, Error};
use crate::posts::model::{Post, PostRequest};

#[get("/posts")]
async fn get_all_opposts(db_pool: web::Data<Pool<Postgres>>) -> impl Responder {
    let result = Post::find_all_opposts(db_pool.get_ref()).await;
    match result {
        Ok(opposts) => HttpResponse::Ok().json(opposts),
        _ => HttpResponse::BadRequest().body("Posts not found.")
    }
}

#[get("/posts/find/{id}")]
async fn get_post_by_id(db_pool: web::Data<Pool<Postgres>>, id: web::Path<i32>) -> impl Responder {
    let result = Post::find_by_id(db_pool.get_ref(), id.into_inner()).await;
    match result {
        Ok(oppost) => HttpResponse::Ok().json(oppost),
        Err(Error::RowNotFound) => HttpResponse::NotFound().body("Post not found."),
        _ => HttpResponse::BadRequest().body("Bad request.")
    }
}

#[post("/posts/add")]
async fn create_post(db_pool: web::Data<Pool<Postgres>>, post: web::Json<PostRequest>) -> impl Responder {
    let result = Post::create(db_pool.get_ref(), post.into_inner()).await;
    match result {
        Ok(post) => HttpResponse::Ok().json(post),
        _ => HttpResponse::BadRequest().body("Bad request.")
    }
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(get_post_by_id);
    config.service(get_all_opposts);
    config.service(create_post);
}