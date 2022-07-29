use async_graphql::*;

use sqlx::SqlitePool;

pub struct QueryRoot;

use crate::core::{session::Credential, sql::query_topic};

use super::{
    topic,
    user::{User, UserBy},
};

#[Object]
impl QueryRoot {
    async fn user(&self, _by: UserBy) -> Result<User> {
        Err(Error::new("unimplemented"))
    }
    async fn topics(
        &self,
        _ctx: &Context<'_>,
        _topic_id: i64,
        #[graphql(default = 10)] _limit: i64,
        #[graphql(default = 0)] _offset: i64,
    ) -> Result<Vec<topic::Topic>> {
        Err(Error::new("unimplemented"))
    }

    async fn topic(&self, ctx: &Context<'_>, topic_id: i64) -> Result<Option<topic::Topic>> {
        let pool = ctx.data::<SqlitePool>().unwrap();
        let cred = ctx.data::<Credential>().unwrap();

        query_topic(pool, cred.0.as_ref().map(|d| d.user_id), topic_id)
            .await
            .map_err(Error::from)
    }
}
