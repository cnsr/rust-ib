// STD
use std::env;
use std::path::Path;

// cargo dependencies
use dotenv::dotenv;
use listenfd::ListenFd;
use actix_web::{web, guard,  App, HttpResponse, HttpServer, Responder};
use sqlx::{migrate, Pool, Postgres, postgres::PgPoolOptions, migrate::Migrator};
use anyhow::Result;

//auth
use actix_web_httpauth::extractors::{AuthenticationError, basic::{BasicAuth, Config}};
use actix_web_httpauth::middleware::HttpAuthentication;

// macro cargo dependencies
#[macro_use]
extern crate lazy_static;

// modules
mod posts;
mod boards;
mod utils;
mod admin;
mod middlewares;

// #[actix_web::main]
#[actix_rt::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let mut listenfd = ListenFd::from_env();

    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url).await?;

    let mut server = HttpServer::new(move || {
        // middlewares need to be created inside the move
        let auth = HttpAuthentication::bearer(middlewares::auth::basic_auth_validator);
        App::new()
            .data(db_pool.clone()) // pass database pool to application so we can access it inside handlers
            .configure(posts::init_routes) // init posts routes
            .configure(boards::init_routes) // init boards routes
            .configure(admin::init_unprotected_routes)
            .service(web::scope("/admin/")
                .wrap(auth)
                .configure(admin::init_routes)
            )
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