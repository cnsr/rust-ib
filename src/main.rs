// STD
use std::env;
use std::path::Path;

// cargo dependencies
use dotenv::dotenv;
use listenfd::ListenFd;
use actix_web::{web, guard,  App, HttpResponse, HttpServer, Responder};
use sqlx::{migrate, Pool, Postgres, postgres::PgPoolOptions, migrate::Migrator};
use anyhow::Result;

// modules
mod posts;
mod utils;


#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let mut listenfd = ListenFd::from_env();

    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url).await?;

    let mut server = HttpServer::new(move || {
        App::new()
            .data(db_pool.clone()) // pass database pool to application so we can access it inside handlers
            .configure(posts::init_routes) // init posts routes
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host: String = env::var("HOST").expect("HOST is not set in .env file");
            let port: String = env::var("PORT").expect("PORT is not set in .env file");
        
            server.bind(format!("{}:{}", host, port))?
        }
    };

    server.run().await?;

    Ok(())
}