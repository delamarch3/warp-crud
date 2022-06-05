use crate::handlers::res::*;
use crate::models::comments::Comment;
use comments::DB;
use serde::{Deserialize, Serialize};
use warp::http::status::StatusCode;

#[derive(Serialize, Deserialize, Debug)]
pub struct NewComment {
    pub userid: i32,
    pub postid: i32,
    pub comment: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateComment {
    pub id: i32,
    pub comment: String,
}

pub async fn insert(
    db: DB,
    comment: NewComment,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Err(e) =
        Comment::new(comment.userid, comment.postid, comment.comment)
            .insert(db)
            .await
    {
        eprint!("{}", e);
        return Ok(res(
            Some(e.to_string()),
            vec![],
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    };
    Ok(res(None, vec![], StatusCode::OK))
}

pub async fn get_one_by_id(
    db: DB,
    id: i32,
) -> Result<impl warp::Reply, warp::Rejection> {
    match Comment::get_one_by_id(db, id).await {
        Some(comment) => Ok(res(None, vec![comment], StatusCode::OK)),
        None => Ok(res(
            Some(String::from("No comment found")),
            vec![],
            StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}

pub async fn get_by_postid(
    db: DB,
    postid: i32,
    limit: u32,
    offset: u32,
) -> Result<impl warp::Reply, warp::Rejection> {
    match Comment::get_by_postid(db, postid, limit, offset).await {
        Ok(comments) => Ok(res(None, comments, StatusCode::OK)),
        Err(e) => {
            eprint!("{}", e);
            Ok(res(
                Some(e.to_string()),
                vec![],
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    }
}

pub async fn update(
    db: DB,
    update: UpdateComment,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Err(e) = Comment::update(db, update.id, update.comment).await {
        eprint!("{}", e);
        return Ok(res(
            Some(e.to_string()),
            vec![],
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    };
    Ok(res(None, vec![], StatusCode::OK))
}

pub async fn delete(
    db: DB,
    id: i32,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Err(e) = Comment::delete(db, id).await {
        eprint!("{}", e);
        return Ok(res(
            Some(e.to_string()),
            vec![],
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    };
    Ok(res(None, vec![], StatusCode::OK))
}
