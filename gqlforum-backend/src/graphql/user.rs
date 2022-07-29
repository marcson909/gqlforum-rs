use async_graphql::*;
use sqlx::{prelude::*, sqlite::SqliteRow};

use super::{post::Post, topic::Topic};

#[derive(Debug, OneofObject)]
pub enum UserBy {
    Name(String),
    Id(i64),
}

#[derive(SimpleObject, Debug)]
#[graphql(complex)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub signature: Option<String>,
}

impl<'r> FromRow<'r, SqliteRow> for User {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            name: row.try_get("username")?,
            signature: row.try_get("post_signature")?,
        })
    }
}

#[ComplexObject]
impl User {
    async fn topics(
        &self,
        _ctx: &Context<'_>,
        #[graphql(default = 10)] _limit: i64,
        #[graphql(default = 0)] _offset: i64,
    ) -> Result<Vec<Topic>> {
        Err(Error::new("unimplemented"))
    }

    async fn posts(
        &self,
        _ctx: &Context<'_>,
        #[graphql(default = 10)] _limit: i64,
        #[graphql(default = 0)] _offset: i64,
    ) -> Result<Vec<Post>> {
        Err(Error::new("unimplemented"))
    }
}
