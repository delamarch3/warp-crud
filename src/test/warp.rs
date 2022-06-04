use crate::api::comments_api;
use crate::handlers::comments_handler::{NewComment, UpdateComment};
use crate::handlers::res::Response;
use crate::models::comments::{gen_id, Comment};
use crate::{db, DB_URI, MAX_CONNECTIONS};
use std::str;

const COMMENTS_PATH: &str = "/comments/v1";

#[tokio::test]
async fn insert() {
    let db = db(MAX_CONNECTIONS, DB_URI).await;

    let insert = comments_api::insert(db);

    let request = NewComment {
        postid: gen_id(),
        userid: gen_id(),
        comment: String::from("Comment"),
    };

    let res = warp::test::request()
        .method("POST")
        .path(COMMENTS_PATH)
        .json(&request)
        .reply(&insert)
        .await;

    assert_eq!(res.status(), 200, "Should return 200 OK");

    let res_str = str::from_utf8(&res.body()).unwrap();
    let res: Response = serde_json::from_str(&res_str).unwrap();

    let test_res = Response {
        message: String::from("Success"),
        error: None,
        data: vec![],
    };

    assert_eq!(res, test_res);
}

#[tokio::test]
async fn get_one() {
    let db = db(MAX_CONNECTIONS, DB_URI).await;

    let get_one = comments_api::get_one(db.clone());

    let postid = gen_id();
    let userid = gen_id();
    let comment =
        Comment::new(userid, postid, String::from("This is a new comment"))
            .insert(db)
            .await
            .unwrap();
    let id = comment.id;

    let query = format!("{}?id={}", COMMENTS_PATH, id);
    let res = warp::test::request()
        .method("GET")
        .path(query.as_str())
        .reply(&get_one)
        .await;

    assert_eq!(res.status(), 200, "Should return 200 OK");

    let res_str = str::from_utf8(&res.body()).unwrap();
    let res: Response = serde_json::from_str(&res_str).unwrap();

    let test_res = Response {
        message: String::from("Success"),
        error: None,
        data: vec![comment],
    };

    assert_eq!(res, test_res);
}

#[tokio::test]
async fn get_post_comments() {
    let db = db(MAX_CONNECTIONS, DB_URI).await;

    let get_post_comments = comments_api::get_by_postid(db.clone());

    let mut comments: Vec<Comment> = Vec::new();
    let postid = gen_id();
    for _ in 0..10 {
        let userid = gen_id();
        let comment = Comment::new(
            userid,
            postid,
            format!("This is a comment by user {}", userid),
        )
        .insert(db.clone())
        .await
        .unwrap();
        comments.push(comment)
    }
    comments.reverse();

    let query1 =
        format!("{}?postid={}&limit=5&offset=0", COMMENTS_PATH, postid);
    let query2 =
        format!("{}?postid={}&limit=5&offset=5", COMMENTS_PATH, postid);
    let res1 = warp::test::request()
        .method("GET")
        .path(&query1)
        .reply(&get_post_comments)
        .await;
    let res2 = warp::test::request()
        .method("GET")
        .path(&query2)
        .reply(&get_post_comments)
        .await;

    assert_eq!(res1.status(), 200, "Should return 200 OK");
    assert_eq!(res2.status(), 200, "Should return 200 OK");

    let res1_str = str::from_utf8(&res1.body()).unwrap();
    let res2_str = str::from_utf8(&res2.body()).unwrap();
    let res1: Response = serde_json::from_str(&res1_str).unwrap();
    let res2: Response = serde_json::from_str(&res2_str).unwrap();

    let (page1, page2) = comments.split_at(5);
    let test_res1 = Response {
        message: String::from("Success"),
        error: None,
        data: page1.to_vec(),
    };
    let test_res2 = Response {
        message: String::from("Success"),
        error: None,
        data: page2.to_vec(),
    };

    assert_eq!(res1, test_res1);
    assert_eq!(res2, test_res2);
}

#[tokio::test]
async fn update() {
    let db = db(MAX_CONNECTIONS, DB_URI).await;

    let update = comments_api::update(db.clone());

    let postid = gen_id();
    let userid = gen_id();
    let comment = Comment::new(userid, postid, String::from("Original"))
        .insert(db.clone())
        .await
        .unwrap();
    let id = comment.id;

    let request = UpdateComment {
        id,
        comment: String::from("Edited"),
    };

    let res = warp::test::request()
        .method("PATCH")
        .path(COMMENTS_PATH)
        .json(&request)
        .reply(&update)
        .await;

    assert_eq!(res.status(), 200, "Should return 200 OK");

    let res_str = str::from_utf8(&res.body()).unwrap();
    let res: Response = serde_json::from_str(&res_str).unwrap();

    let test_res = Response {
        message: String::from("Success"),
        error: None,
        data: vec![],
    };

    assert_eq!(res, test_res);
}

#[tokio::test]
async fn delete() {
    let db = db(MAX_CONNECTIONS, DB_URI).await;

    let delete = comments_api::delete(db.clone());

    let postid = gen_id();
    let userid = gen_id();

    let comment = Comment::new(
        userid,
        postid,
        String::from("This comment will be deleted"),
    )
    .insert(db.clone())
    .await
    .unwrap();
    let id = comment.id;

    let query = format!("{}?id={}", COMMENTS_PATH, id);
    let res = warp::test::request()
        .method("DELETE")
        .path(&query)
        .reply(&delete)
        .await;

    assert_eq!(res.status(), 200, "Should return 200 OK");

    let res_str = str::from_utf8(&res.body()).unwrap();
    let res: Response = serde_json::from_str(&res_str).unwrap();

    let test_res = Response {
        message: String::from("Success"),
        error: None,
        data: vec![],
    };

    assert_eq!(res, test_res);

    let comment = Comment::get_one_by_id(db, id).await;
    assert_eq!(comment, None);
}
