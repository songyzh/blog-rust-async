use sqlx::mysql::MySqlRow;
use sqlx::MySqlPool;

pub async fn get_posts(
    mut pool: &MySqlPool,
    offset: &i32,
    limit: &i32,
    tag: &Option<String>,
) -> Vec<MySqlRow> {
    match tag {
        None => sqlx::query("select * from post order by id desc limit ?, ?")
            .bind(offset).bind(limit)
            .fetch_all(&mut pool)
            .await
            .unwrap(),
        Some(tag) => sqlx::query("select p.* from post p join post_tag pt on p.id = pt.post_id join tag t on pt.tag_id = t.id where t.text = ? order by id desc limit ?, ?")
            .bind(tag).bind(offset).bind(limit)
            .fetch_all(&mut pool)
            .await
            .unwrap(),
    }
}

pub async fn count_posts(mut pool: &MySqlPool, tag: &Option<String>) -> MySqlRow {
    match tag {
        None => sqlx::query("select count(1) as cnt from post")
            .fetch_one(&mut pool)
            .await
            .unwrap(),
        Some(tag) => sqlx::query("select count(1) as cnt from post p join post_tag pt on p.id = pt.post_id join tag t on pt.tag_id = t.id where t.text = ?")
            .bind(tag)
            .fetch_one(&mut pool)
            .await
            .unwrap(),
    }
}

pub async fn get_post_by_slug(mut pool: &MySqlPool, slug: &String) -> MySqlRow {
    sqlx::query("select * from post where slug = ?")
        .bind(slug)
        .fetch_one(&mut pool)
        .await
        .unwrap()
}

pub async fn get_all_tags(mut pool: &MySqlPool) -> Vec<MySqlRow> {
    sqlx::query("select t.* from tag t join post_tag pt on t.id = pt.tag_id group by t.id order by t.id asc")
        .fetch_all(&mut pool)
        .await
        .unwrap()
}

pub async fn get_post_tags(mut pool: &MySqlPool, post_id: &i32) -> Vec<MySqlRow> {
    sqlx::query("select t.* from tag t join post_tag pt on t.id = pt.tag_id where pt.post_id = ? group by t.id order by t.id asc")
        .bind(post_id)
        .fetch_all(&mut pool)
        .await
        .unwrap()
}

pub async fn get_family_by_id(mut pool: &MySqlPool, id: &i32) -> MySqlRow {
    sqlx::query("select * from family where id = ?")
        .bind(id)
        .fetch_one(&mut pool)
        .await
        .unwrap()
}
