use sqlx::sqlite::SqliteRow;
use sqlx::{query, query_as, Row, Sqlite, SqliteExecutor, Transaction};

use crate::core::session::UserCredential;
use crate::graphql::topic::Topic;

use super::new_post;

pub async fn query_topic_by_id<'e, E: SqliteExecutor<'e>>(
    pool: E,
    cred: &UserCredential,
    topic_id: i64,
) -> Result<Option<Topic>, sqlx::Error> {
    let topic = query_as(include_str!("topic_by_id.sql"))
        .bind(cred.user_id())
        .bind(topic_id)
        .fetch_optional(pool)
        .await?;
    Ok(topic)
}

pub async fn new_topic(
    tx: &mut Transaction<'_, Sqlite>,
    user_id: i64,
    title: String,
    body: String,
) -> Result<i64, sqlx::Error> {
    let topic_id: i64 = query(
        r#"
    INSERT INTO topics (author_user_id, title)
    VALUES (?1, ?2) RETURNING id
    "#,
    )
    .bind(user_id)
    .bind(title)
    .map(|row: SqliteRow| row.get("id"))
    .fetch_one(&mut *tx)
    .await?;
    new_post(&mut *tx, user_id, topic_id, body).await?;
    Ok(topic_id)
}
