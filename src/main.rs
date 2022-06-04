use std::env;
use warp::Filter;

mod api;
mod handlers;
mod models;

use self::api::comments_api;
use comments::consts::*;
use comments::db;

#[cfg(test)]
mod test;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let (uri, max_connections) = get_env();
    let db = db(max_connections, uri.as_str()).await;
    
    let api = comments_api::api(db);
    let filter = api.with(warp::log("api"));
    warp::serve(filter).run(([0, 0, 0, 0], 8000)).await;
    Ok(())
}

fn get_env() -> (String, u32) {
    let uri = match env::var("DB_URI") {
        Ok(uri) => uri,
        Err(_) => DB_URI.to_owned(),
    };
    let max_connections = match env::var("MAX_CONNECTIONS") {
        Ok(max_connections) => max_connections.parse::<u32>().unwrap(),
        Err(_) => MAX_CONNECTIONS
    };
    (uri, max_connections)
}