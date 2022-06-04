use crate::models::comments::{gen_id, Comment};
use comments::consts::*;
use comments::db;

#[tokio::test]
async fn insert_and_get_one() {
    let db = db(MAX_CONNECTIONS, DB_URI).await;

    let postid = gen_id();
    let userid = gen_id();

    let comment = Comment::new(userid, postid, String::from("Comment"))
        .insert(db.clone())
        .await
        .unwrap();
    let id = comment.id;

    let db_comment = Comment::get_one_by_id(db, id).await.unwrap();

    assert_eq!(comment, db_comment);
}

#[tokio::test]
async fn delete_and_get_none() {
    let db = db(MAX_CONNECTIONS, DB_URI).await;

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

    Comment::delete(db.clone(), id).await.unwrap();

    let comment_option = Comment::get_one_by_id(db, id).await;

    assert_eq!(comment_option, None);
}

#[tokio::test]
async fn update() {
    let db = db(MAX_CONNECTIONS, DB_URI).await;

    let postid = gen_id();
    let userid = gen_id();
    let original = "Original";
    let edited = "Edited";

    let comment = Comment::new(userid, postid, String::from(original))
        .insert(db.clone())
        .await
        .unwrap();

    let id = comment.id;
    let date = comment.date.to_owned();
    let timestamp = comment.timestamp;

    Comment::update(db.clone(), id, String::from(edited))
        .await
        .unwrap();

    let db_comment = Comment::get_one_by_id(db, id).await.unwrap();
    let test_comment = Comment {
        id,
        userid,
        postid,
        likes: 0,
        comment: String::from(edited),
        date,
        timestamp,
    };

    assert_eq!(test_comment, db_comment);
}

#[tokio::test]
async fn pagination() {
    let db = db(MAX_CONNECTIONS, DB_URI).await;

    let postid = gen_id();
    let limit = 5;

    let mut comments: Vec<Comment> = Vec::new();
    for _ in 0..limit * 2 {
        let userid = gen_id();
        let comment = Comment::new(
            userid,
            postid,
            format!("This is a comment by user {}", userid),
        )
        .insert(db.clone())
        .await
        .unwrap();
        comments.push(comment);
    }
    comments.reverse();
    let (page1, page2) = comments.split_at(5);

    let db_page_1 =
        Comment::get_by_postid(db.clone(), postid, limit, 0 * limit)
            .await
            .unwrap();
    let db_page_2 = Comment::get_by_postid(db, postid, limit, 1 * limit)
        .await
        .unwrap();

    assert_eq!(page1.to_vec(), db_page_1);
    assert_eq!(page2.to_vec(), db_page_2);
}
