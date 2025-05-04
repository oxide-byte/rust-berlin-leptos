use crate::config::connect_db;
use crate::graphql::{MeetupUrlCount, MeetupUrlFilter, MeetupUrlResponse, Page, ServerContext};
use crate::repository::{count_url, select_url};
use juniper::graphql_object;
use tracing::log::{log, Level};

#[derive(Clone, Copy, Debug)]
pub struct Query;

#[graphql_object]
#[graphql_object(context = ServerContext)]
impl Query {
    async fn meetup_url_list(
        #[graphql(context)] _server_context: &ServerContext,
        filter: MeetupUrlFilter) -> MeetupUrlResponse {
        log!(Level::Info, "Received request query: {:?}", filter);

        let client = connect_db().await;
        let result = select_url(&client, filter.clone())
            .await
            .unwrap_or(Vec::new());

        let count = count_url(&client, filter).await.unwrap_or(0);

        MeetupUrlResponse {
            result,
            page: Page {
                size: 0,
                current: 0,
                total: count,
            },
        }
    }

    async fn meetup_url_count(
        #[graphql(context)] _server_context: &ServerContext,
        filter: MeetupUrlFilter) -> MeetupUrlCount {
        log!(Level::Info, "Received request count: {:?}", filter);

        let client = connect_db().await;
        let result = count_url(&client, filter).await;

        MeetupUrlCount { count: result.unwrap_or(0) as i32 }
    }
}