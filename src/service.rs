use crate::db;

use chrono::{DateTime, FixedOffset, NaiveDateTime, TimeZone};
use chrono::{Duration, Utc};
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use serde_json::Value;
use sqlx::mysql::MySqlRow;
use sqlx::{MySqlPool, Row};
use std::env;
use tide::{Request, Response};

pub async fn get_posts(mut req: Request<MySqlPool>) -> Response {
    let pool = req.state();
    let pagination: Pagination = req.query().unwrap();
    let rows = db::get_posts(
        pool,
        &((pagination.page - 1) * pagination.size),
        &pagination.size,
        &pagination.tag,
    )
    .await;
    let mut posts = vec![];
    for row in rows {
        posts.push(parse_post(pool, &row).await)
    }
    let total: i32 = db::count_posts(pool, &pagination.tag).await.get("cnt");
    let total_page: i32 = match total % &pagination.size {
        0 => total / &pagination.size,
        _ => total / &pagination.size + 1,
    };
    let resp = Resp {
        data: json!({
            "posts": posts ,
            "total": total,
            "total_page": total_page,
        }),
        ..Default::default()
    };
    Response::new(200).body_json(&resp).unwrap()
}

pub async fn get_post_by_slug(req: Request<MySqlPool>) -> Response {
    let pool = req.state();
    let slug: String = req.param("slug").unwrap();
    let row = db::get_post_by_slug(pool, &slug).await;
    let resp = Resp {
        data: parse_post(pool, &row).await,
        ..Default::default()
    };
    Response::new(200).body_json(&resp).unwrap()
}

pub async fn get_tags(req: Request<MySqlPool>) -> Response {
    let pool = req.state();
    let rows = db::get_all_tags(pool).await;
    let tags: Vec<String> = rows.iter().map(|row| row.get("text")).collect();
    let resp = Resp {
        data: json!(tags),
        ..Default::default()
    };
    Response::new(200).body_json(&resp).unwrap()
}

pub async fn benchmark(mut req: Request<MySqlPool>) -> Response {
    let resp = Resp {
        ..Default::default()
    };
    Response::new(200).body_json(&resp).unwrap()
}

pub async fn parse_post(pool: &MySqlPool, row: &MySqlRow) -> Value {
    let id: i32 = row.get("id");
    let family_id: i32 = row.get("family_id");
    let title: String = row.get("title");
    let slug: String = row.get("slug");
    let cover: String = row.get("cover");
    let content: String = row.get("content");
    let tags: Vec<String> = db::get_post_tags(pool, &id)
        .await
        .iter()
        .map(|row| row.get("text"))
        .collect();
    let family = db::get_family_by_id(pool, &family_id).await;
    let family_avatar: String = family.get("avatar");
    let family_name: String = family.get("name");
    let created_at: NaiveDateTime = row.get("created_at");
    let updated_at: NaiveDateTime = row.get("updated_at");
    let tz_offset = FixedOffset::east(8 * 3600);
    let created_at: DateTime<FixedOffset> = tz_offset.from_local_datetime(&created_at).unwrap();
    let updated_at: DateTime<FixedOffset> = tz_offset.from_local_datetime(&updated_at).unwrap();

    json!({
        "title": title,
        "slug": slug,
        "cover": cover,
        "content": content,
        "tags": tags,
        "family": json!({
            "avatar": family_avatar,
            "name": family_name,
        }),
        "created_at": created_at,
        "updated_at": updated_at,
    })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pagination {
    pub page: i32,
    pub size: i32,
    pub tag: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Resp {
    code: i32,
    data: Value,
    msg: String,
}

impl Default for Resp {
    fn default() -> Self {
        Resp {
            code: 0,
            data: Value::Null,
            msg: String::from("ok"),
        }
    }
}
