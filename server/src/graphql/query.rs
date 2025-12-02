use crate::config::connect_db;
use crate::graphql::{MeetupUrlCount, MeetupUrlFilter, MeetupUrlResponse, Page, ServerContext};
use crate::repository::{count_url, select_url};
use async_graphql::{Context, Object};
use tracing::log::{log, Level};

#[derive(Clone, Copy, Debug)]
pub struct Query;

#[Object]
impl Query {
    async fn meetup_url_list(&self, ctx: &Context<'_>, filter: MeetupUrlFilter) -> MeetupUrlResponse {
        log!(Level::Info, "Received request query: {:?}", filter);

        // Access shared context if needed
        let _server_context = ctx.data_unchecked::<ServerContext>();

        let client = connect_db().await;
        let result = select_url(&client, filter.clone())
            .await
            .unwrap_or(Vec::new());

        let count = count_url(&client, filter).await;

        MeetupUrlResponse {
            result,
            page: Page {
                size: 0,
                current: 0,
                total: count.unwrap_or(0),
            },
        }
    }

    async fn meetup_url_count(&self, ctx: &Context<'_>, filter: MeetupUrlFilter) -> MeetupUrlCount {
        log!(Level::Info, "Received request count: {:?}", filter);

        let _server_context = ctx.data_unchecked::<ServerContext>();

        let client = connect_db().await;
        let result = count_url(&client, filter).await;

        MeetupUrlCount { count: result.unwrap_or(0) as i32 }
    }
}