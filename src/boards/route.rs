use crate::boards::*;
use actix_web::{get, post, delete, put, web, HttpResponse, Responder};
use sqlx::{Postgres, Pool, Error};


#[get("/boards/")]
async fn get_all_boards(db_pool: web::Data<Pool<Postgres>>) -> impl Responder {
    let result = Board::find_all_visible_boards(db_pool.get_ref()).await;
    match result {
        Ok(boards) => HttpResponse::Ok().json(boards),
        _ => HttpResponse::BadRequest().body("Bad request.")
    }
}


pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(get_all_boards);
}