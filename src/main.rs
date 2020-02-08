use chrono::{Duration, Utc};
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use sqlx::mysql::MySqlRow;
use sqlx::{MySqlPool, Row};
use std::env;
use tide::{Request, Response};

mod db;
mod service;

#[async_std::main]
async fn main() {
    let pool = MySqlPool::new(env::var("BLOG_DATABASE_RUST").unwrap().as_str())
        .await
        .unwrap();

    let mut server = tide::with_state(pool);

    server.at("/benchmark").get(service::benchmark);
    server.at("/api").nest(|server| {
        server.at("/posts").nest(|server| {
            server.at("").get(service::get_posts);
            server.at("/:slug").get(service::get_post_by_slug);
        });
        server.at("/tags").get(service::get_tags);
    });

    server.listen(("localhost", 3333)).await.unwrap();
}
