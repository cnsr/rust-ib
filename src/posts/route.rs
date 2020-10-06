use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use sqlx::{Postgres, Pool, Error};
use crate::posts::model::{Post, PostRequest};

// for future reference
// Err(Error::RowNotFound) => HttpResponse::NotFound().body("Post not found."),

// gets all opposts, no board
#[get("/posts")]
async fn get_all_opposts(db_pool: web::Data<Pool<Postgres>>) -> impl Responder {
    let result = Post::find_all_opposts(db_pool.get_ref()).await;
    match result {
        Ok(opposts) => HttpResponse::Ok().json(opposts),
        _ => HttpResponse::BadRequest().body("Posts not found.")
    }
}

// gets all posts in a thread
#[get("/posts/find/{id}")]
async fn find_by_oppost_id(db_pool: web::Data<Pool<Postgres>>, id: web::Path<i32>) -> impl Responder {
    let result = Post::find_by_oppost_id(db_pool.get_ref(), id.into_inner()).await;
    match result {
        Ok(posts) => HttpResponse::Ok().json(posts),
        _ => HttpResponse::BadRequest().body("Bad request.")
    }
}

#[get("/posts/board/{id}")]
async fn get_opposts_by_board_id(db_pool: web::Data<Pool<Postgres>>, id: web::Path<i32>) -> impl Responder {
    let result = Post::find_opposts_by_board_id(db_pool.get_ref(), id.into_inner()).await;
    match result {
        Ok(posts) => HttpResponse::Ok().json(posts),
        _ => HttpResponse::BadRequest().body("Bad request.")
    }
}

// create_post and create_post_in_thread shouldnt be two different functions
#[post("/posts/add")]
async fn create_post(db_pool: web::Data<Pool<Postgres>>, post: web::Json<PostRequest>) -> impl Responder {
    let result = Post::create(
        db_pool.get_ref(),
        post.into_inner(),
        None
    ).await;
    match result {
        Ok(post) => HttpResponse::Ok().json(post),
        _ => HttpResponse::BadRequest().body("Bad request.")
    }
}

#[post("/posts/add/{oppost_id}")]
async fn create_post_in_thread(
    db_pool: web::Data<Pool<Postgres>>,
    post: web::Json<PostRequest>,
    oppost_id: web::Path<i32>) -> impl Responder {
        let result = Post::create(
            db_pool.get_ref(),
            post.into_inner(),
            Some(oppost_id.into_inner())
        ).await;
        match result {
            Ok(post) => HttpResponse::Ok().json(post),
            _ => HttpResponse::BadRequest().body("Bad request.")
        }
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_by_oppost_id);
    config.service(get_all_opposts);
    config.service(get_opposts_by_board_id);
    config.service(create_post);
    config.service(create_post_in_thread);
}