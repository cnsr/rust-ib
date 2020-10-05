use serde::{Deserialize, Serialize};
use actix_web::{HttpResponse, HttpRequest, Responder, Error};
use futures::future::{ready, Ready};
use sqlx::{postgres::{PgPoolOptions, PgRow}, query_as};
use sqlx::query::Query;
use sqlx::{FromRow, Row, Pool, Postgres};
use anyhow::Result;
use crate::utils::get_unix_timestamp_ms;

// for user input
#[derive(Deserialize, Serialize)]
pub struct PostRequest {
    pub is_oppost: bool,
    pub subject: Option<String>,
    pub body: Option<String>,
    pub board_id: i32
}

// db representation
#[derive(Serialize, FromRow)]
pub struct Post {
    pub id: i32,
    pub is_oppost: bool,
    pub subject: Option<String>,
    pub body: Option<String>,
    pub created_at: i64,
    pub board_id: i32
} 

// implementation of Responder for Post to return Post from action handler
impl Responder for Post {
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

impl Post {
    pub async fn find_all_opposts(pool: &Pool<Postgres>) -> Result<Vec<Post>> {
        let mut posts: Vec<Post> = vec![];
        let records = query_as::<_, Post>(
            r#"
                SELECT
                id, is_oppost, subject, body, subject, body, created_at
                FROM posts
                WHERE (is_oppost = TRUE)
                ORDER BY created_at;
            "#
        ).fetch_all(pool).await?;

        for record in records {
            // this really need a simpler conversion method
            posts.push(Post {
                id: record.id,
                is_oppost: record.is_oppost,
                body: Some(record.body.unwrap()),
                subject: Some(record.subject.unwrap()),
                created_at: record.created_at,
                board_id: record.board_id
            });
        }

        Ok(posts)
    }

    pub async fn find_by_id(pool: &Pool<Postgres>, id: i32) -> Result<Post, sqlx::Error> {
        let mut tx = pool.begin().await?; // transaction
        let post = sqlx::query_as::<_, Post>(
            r#"
                SELECT id, is_oppost, subject, body, subject, body, created_at, board_id
                FROM posts
                WHERE (id = $1);
            "#
        ).bind(&id).fetch_one(&mut tx).await?;

        Ok(post)

    }

    pub async fn find_opposts_by_board_id(pool: &Pool<Postgres>, board_id: i32) -> Result<Vec<Post>, sqlx::Error> {
        let mut posts: Vec<Post> = vec![];
        let records = sqlx::query_as::<_, Post>(
            r#"
                SELECT id, is_oppost, subject, body, subject, body, created_at, board_id
                FROM posts
                WHERE (board_id = $1, oppost = TRUE);
            "#
        ).bind(&board_id).fetch_all(pool).await?;

        for record in records {
            posts.push(Post {
                id: record.id,
                is_oppost: record.is_oppost,
                body: Some(record.body.unwrap()),
                subject: Some(record.subject.unwrap()),
                created_at: record.created_at,
                board_id: record.board_id
            });
        }

        Ok(posts)

    }

    pub async fn create(pool: &Pool<Postgres>, post: PostRequest) -> Result<Post> {
        let mut tx = pool.begin().await?; // transaction
        let post = sqlx::query_as::<_, Post>(
            r#"INSERT INTO posts
            (is_oppost, subject, body, created_at, board_id)
            VALUES
            ($1, $2, $3, $4, $5)
            RETURNING id, is_oppost, subject, body, created_at, board_id
            "#
        )
            .bind(post.is_oppost)
            .bind(&post.subject.unwrap())
            .bind(&post.body.unwrap())
            .bind(&get_unix_timestamp_ms())
            .bind(&post.board_id)
            .fetch_one(&mut tx)
            .await?;

        tx.commit().await?;
        Ok(post)
    }
}