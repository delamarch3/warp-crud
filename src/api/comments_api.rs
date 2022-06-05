use crate::handlers::comments_handler;
use comments::DB;
use serde::Deserialize;
use warp::filters::BoxedFilter;
use warp::{path, Filter};

#[derive(Deserialize)]
pub struct GetOne {
    pub id: i32,
}

#[derive(Deserialize)]
pub struct Get {
    pub postid: i32,
    pub limit: u32,
    pub offset: u32,
}

#[derive(Deserialize)]
pub struct Delete {
    pub id: i32,
}

fn path_prefix() -> BoxedFilter<()> {
    path!("comments" / "v1").boxed()
}

pub fn api(
    db: DB,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    health()
        .or(insert(db.clone()))
        .or(get_one(db.clone()))
        .or(get_by_postid(db.clone()))
        .or(update(db.clone()))
        .or(delete(db))
}

fn health(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get().and(path!("health")).and_then(|| async {
        Ok::<_, warp::Rejection>(warp::reply::with_status(
            "OK",
            warp::http::status::StatusCode::OK,
        ))
    })
}

pub fn insert(
    db: DB,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let json =
        warp::body::content_length_limit(1024 * 16).and(warp::body::json());

    warp::post()
        .and(path_prefix())
        .and(warp::path::end())
        .and(warp::any().map(move || db.clone()))
        .and(json)
        .and_then(comments_handler::insert)
}

pub fn get_one(
    db: DB,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(path_prefix())
        .and(warp::path::end())
        .and(warp::any().map(move || db.clone()))
        .and(warp::query().map(|q: GetOne| q.id))
        .and_then(comments_handler::get_one_by_id)
}

pub fn get_by_postid(
    db: DB,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(path_prefix())
        .and(warp::path::end())
        .and(warp::any().map(move || db.clone()))
        .and(warp::query().map(|q: Get| q.postid))
        .and(warp::query().map(|q: Get| q.limit))
        .and(warp::query().map(|q: Get| q.offset))
        .and_then(comments_handler::get_by_postid)
}

pub fn update(
    db: DB,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let json =
        warp::body::content_length_limit(1024 * 16).and(warp::body::json());

    warp::patch()
        .and(path_prefix())
        .and(warp::path::end())
        .and(warp::any().map(move || db.clone()))
        .and(json)
        .and_then(comments_handler::update)
}

pub fn delete(
    db: DB,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::delete()
        .and(path_prefix())
        .and(warp::path::end())
        .and(warp::any().map(move || db.clone()))
        .and(warp::query().map(|q: Delete| q.id))
        .and_then(comments_handler::delete)
}
