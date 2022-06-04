use comments::DB;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, sqlx::FromRow, PartialEq, Debug, Clone)]
pub struct Comment {
    pub id: i32,
    pub userid: i32,
    pub postid: i32,
    pub likes: i32,
    pub comment: String,
    pub date: String,
    pub timestamp: i64,
}

pub fn gen_id() -> i32 {
    let mut rng = thread_rng();
    let num: i32 = rng.gen();
    num.abs()
}

fn get_timestamp() -> i64 {
    let start = SystemTime::now();
    let since_epoch = start.duration_since(UNIX_EPOCH).unwrap();
    since_epoch.as_millis() as i64
}

impl Comment {
    pub fn new(userid: i32, postid: i32, comment: String) -> Self {
        let id = gen_id();
        Self {
            id,
            userid,
            postid,
            likes: 0,
            comment,
            date: String::from(""),
            timestamp: get_timestamp(),
        }
    }

    pub async fn insert(self, db: DB) -> Result<Self, sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO comments (id, userid, postid, likes, date, comment, timestamp)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
        .bind(&self.id)
        .bind(&self.userid)
        .bind(&self.postid)
        .bind(&self.likes)
        .bind(&self.date)
        .bind(&self.comment)
        .bind(&self.timestamp)
        .execute(&*db)
        .await?;
        Ok(self)
    }

    pub async fn get_one_by_id(db: DB, id: i32) -> Option<Comment> {
        let comment = sqlx::query_as(
            r#"
            SELECT * FROM comments WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_one(&*db)
        .await;

        match comment {
            Ok(comment) => Some(comment),
            Err(e) => {
                eprintln!("{}", e);
                None
            }
        }
    }

    pub async fn get_by_postid(
        db: DB,
        id: i32,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<Comment>, sqlx::Error> {
        let result = sqlx::query_as(
            r#"
            SELECT * FROM comments WHERE postid = $1
            ORDER BY comments.timestamp DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&*db)
        .await?;

        Ok(result)
    }

    pub async fn update(
        db: DB,
        id: i32,
        comment: String,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE comments SET comment = $1 WHERE id = $2
            "#,
        )
        .bind(comment)
        .bind(id)
        .execute(&*db)
        .await?;

        Ok(())
    }

    pub async fn delete(db: DB, id: i32) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            DELETE FROM comments WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&*db)
        .await?;

        Ok(())
    }
}
