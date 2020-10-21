use serde::{Deserialize, Serialize};
use actix_web::{HttpResponse, HttpRequest, Responder, Error};
use futures::future::{ready, Ready};
use sqlx::{postgres::{PgPoolOptions, PgRow, PgDone}, query_as};
use sqlx::query::Query;
use sqlx::{FromRow, Row, Pool, Postgres, query};
use anyhow::Result;
use crate::utils::get_unix_timestamp_ms;

const MAX_POSTS_IN_TREAD: i64 = 50; // TODO: fetch from board we are currently using

// for user input
#[derive(Deserialize, Serialize)]
pub struct PostRequest {
    pub subject: Option<String>,
    pub body: Option<String>,
    pub board_id: i32
}

// db representation
#[derive(Serialize, FromRow, Clone)]
pub struct Post {
    pub id: i32,
    pub is_oppost: bool,
    pub subject: Option<String>,
    pub body: Option<String>,
    pub created_at: i64,
    pub board_id: i32,
    pub oppost_id: Option<i32>,
    pub is_locked: bool,
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
                board_id: record.board_id,
                oppost_id: None,
                is_locked: record.is_locked,
            });
        }

        Ok(posts)
    }

    pub async fn find_by_id(pool: &Pool<Postgres>, id: i32) -> Result<Post, sqlx::Error> {
        let mut tx = pool.begin().await?; // transaction
        let post = sqlx::query_as::<_, Post>(
            r#"
                SELECT id, is_oppost, subject, body, subject, body, created_at, board_id, is_locked
                FROM posts
                WHERE (id = $1);
            "#
        ).bind(&id).fetch_one(&mut tx).await?;

        Ok(post)

    }

    // get number of posts in a thread
    pub async fn count_by_oppost_id(post: Self, pool: &Pool<Postgres>) -> Result<i64, sqlx::Error> {
        let oppost_id: i32 = if post.is_oppost {post.id} else {post.oppost_id.unwrap()};
        let count = sqlx::query!(
            "SELECT COUNT(*) as count FROM posts WHERE (oppost_id = $1);", oppost_id
        ).fetch_all(pool).await?;

        // holy fuck wtf is this
        Ok(count[0].count.unwrap())
    }

    pub async fn find_by_oppost_id(pool: &Pool<Postgres>, oppost_id: i32) -> Result<Vec<Post>, sqlx::Error> {
        let mut posts: Vec<Post> = vec![];
        let records = query_as::<_, Post>(
            r#"
                SELECT id, is_oppost, subject, body, subject, body, created_at, is_locked
                FROM posts WHERE (id = $1)
                UNION
                SELECT id, is_oppost, subject, body, subject, body, created_at, FALSE as is_locked
                FROM posts WHERE (oppost_id = $1)
                ORDER BY created_at;
            "#
        ).bind(&oppost_id).fetch_all(pool).await?;

        for record in records {
            // this really need a simpler conversion method
            posts.push(Post {
                id: record.id,
                is_oppost: record.is_oppost,
                body: Some(record.body.unwrap()),
                subject: Some(record.subject.unwrap()),
                created_at: record.created_at,
                board_id: record.board_id,
                oppost_id: Some(oppost_id),
                is_locked: record.is_locked,
            });
        }

        Ok(posts)
    }

    pub async fn find_opposts_by_board_id(pool: &Pool<Postgres>, board_id: i32) -> Result<Vec<Post>, sqlx::Error> {
        let mut posts: Vec<Post> = vec![];
        let records = sqlx::query_as::<_, Post>(
            r#"
                SELECT id, is_oppost, subject, body, subject, body, created_at, board_id, is_locked
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
                board_id: record.board_id,
                oppost_id: None,
                is_locked: record.is_locked
            });
        }

        Ok(posts)

    }

    pub async fn create(pool: &Pool<Postgres>, post: PostRequest, oppost_id: Option<i32>) -> Result<Post> {
        let mut tx = pool.begin().await?; // transaction
        let result: Post;
        match oppost_id {
            Some(op ) => {
                result = sqlx::query_as::<_, Post>(
                    r#"INSERT INTO posts
                    (is_oppost, subject, body, created_at, board_id, oppost_id)
                    VALUES
                    ($1, $2, $3, $4, $5, $6)
                    RETURNING id, is_oppost, subject, body, created_at, board_id
                    "#
                )
                .bind(false) // if oppost_id is supplied - always false
                .bind(&post.subject.unwrap())
                .bind(&post.body.unwrap())
                .bind(&get_unix_timestamp_ms())
                .bind(&post.board_id)
                .bind(&op)
                .fetch_one(&mut tx)
                .await?;
            },
            None => {
                result = sqlx::query_as::<_, Post>(
                    r#"INSERT INTO posts
                    (is_oppost, subject, body, created_at, board_id)
                    VALUES
                    ($1, $2, $3, $4, $5)
                    RETURNING id, is_oppost, subject, body, created_at, board_id
                    "#
                )
                .bind(true) // with oppost_id - always true
                .bind(&post.subject.unwrap())
                .bind(&post.body.unwrap())
                .bind(&get_unix_timestamp_ms())
                .bind(&post.board_id)
                .fetch_one(&mut tx)
                .await?;
            }
        };
        

        tx.commit().await?;
        Ok(result)
    }

    // check if thread should be locked and lock it if necessary
    pub async fn verify_thread(post: Self, pool: &Pool<Postgres>) -> Result<bool, sqlx::Error> {
        let oppost_id: i32 = if post.is_oppost {post.id} else {post.oppost_id.unwrap()};
        let posts_in_thread = Post::count_by_oppost_id(post, pool).await?;
        if posts_in_thread >= MAX_POSTS_IN_TREAD {
            Post::lock_thread(oppost_id.into(), pool).await?;
            return Ok(true)
        }
        Ok(true)
    }

    pub async fn lock_thread(oppost_id: i64, pool: &Pool<Postgres>) -> sqlx::Result<PgDone> {
        let mut tx = pool.begin().await.unwrap(); // transaction
        sqlx::query!(
            r#"
                UPDATE posts
                SET is_locked = TRUE
                WHERE (oppost_id = $1);
            "#, oppost_id as i32
        ).execute(&mut tx).await //fetch_one(&mut tx).await?;
    }

}