use crate::models::comments::Comment;
use serde::{Deserialize, Serialize};
use warp::http::status::StatusCode;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Response {
    pub message: String,
    pub error: Option<String>,
    pub data: Vec<Comment>,
}

pub fn res(
    error: Option<String>,
    data: Vec<Comment>,
    status_code: StatusCode,
) -> impl warp::Reply {
    const SUCCESS: &str = "Success";
    const ERROR: &str = "ERROR";

    let res = match error {
        Some(e) => Response {
            message: String::from(ERROR),
            error: Some(e),
            data,
        },
        None => Response {
            message: String::from(SUCCESS),
            error: None,
            data,
        },
    };

    let reply = warp::reply::json(&res);
    warp::reply::with_status(reply, status_code)
}
