use serde::{Serialize, Deserialize};
use actix_web::{HttpResponse, HttpRequest, Responder, Error};
use futures::future::{ready, Ready};
use sqlx::{postgres::{PgPoolOptions, PgRow}, query_as};
use sqlx::{FromRow, Row, Pool, Postgres};
use anyhow::Result;

use crate::utils::get_unix_timestamp_ms;

#[derive(Deserialize, Serialize)]
pub struct BoardRequest {
    short: String,
    long: String,
    description: Option<String>,
    is_hidden: bool
}

#[derive(FromRow, Serialize)]
pub struct Board {
    board_id: i32,
    short: String,
    long: String,
    description: Option<String>,
    created_at: i64,
    is_hidden: bool,
    max_posts: i32,
}

impl Responder for Board {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        ready(Ok(
            HttpResponse::Ok()
                .content_type("application/json")
                .body(body)
        ))
    }
}

impl Board {
    pub async fn find_all_boards(pool: &Pool<Postgres>) -> Result<Vec<Board>> {
        let mut boards: Vec<Board> = vec![];
        let records = query_as::<_, Board>(
            r#"
            SELECT
                board_id, short, long, description, created_at, is_hidden
                FROM posts
                ORDER BY created_at;
            "#
        ).fetch_all(pool).await?;

        for record in records {
            boards.push(Board {
                board_id: record.board_id,
                short: record.short,
                long: record.long,
                description: Some(record.description.unwrap()),
                created_at: record.created_at,
                is_hidden: record.is_hidden,
                max_posts: record.max_posts
            });
        }

        Ok(boards)
    }

    pub async fn find_all_visible_boards(pool: &Pool<Postgres>) -> Result<Vec<Board>> {
        let mut boards: Vec<Board> = vec![];
        let records = query_as::<_, Board>(
            r#"
            SELECT
                board_id, short, long, description, created_at, is_hidden, max_posts
                FROM posts
                WHERE (is_hidden = FALSE)
                ORDER BY created_at;
            "#
        ).fetch_all(pool).await?;

        for record in records {
            boards.push(Board {
                board_id: record.board_id,
                short: record.short,
                long: record.long,
                description: Some(record.description.unwrap()),
                created_at: record.created_at,
                is_hidden: record.is_hidden,
                max_posts: record.max_posts,
            });
        }

        Ok(boards)
    }

    pub async fn find_by_short(pool: &Pool<Postgres>, short: String) -> Result<Board, sqlx::Error> {
        let mut tx = pool.begin().await?; // transaction
        let board = sqlx::query_as::<_, Board>(
            r#"
                board_id, short, long, description, created_at, is_hidden, max_posts
                FROM posts
                WHERE (short = $1);
            "#
        ).bind(&short).fetch_one(&mut tx).await?;
        Ok(board)
    }

    pub async fn find_by_id(pool: &Pool<Postgres>, board_id: i64) -> Result<Board, sqlx::Error> {
        let mut tx = pool.begin().await?; // transaction
        let board = sqlx::query_as::<_, Board>(
            r#"
                board_id, short, long, description, created_at, is_hidden, max_posts
                FROM posts
                WHERE (board_id = $1);
            "#
        ).bind(&board_id).fetch_one(&mut tx).await?;
        Ok(board)
    }

    pub async fn max_posts(pool: &Pool<Postgres>, board_id: i64) -> Result<i64, sqlx::Error> { 
        let board = Self::find_by_id(pool, board_id).await?;
        return Ok(board.max_posts.into())
    }
}