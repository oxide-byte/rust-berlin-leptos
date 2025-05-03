use juniper::graphql_object;
use tracing::log::{Level, log};
use crate::config::connect_db;
use crate::graphql::{MeetupUrl, MeetupUrlCount, MeetupUrlFilter, ServerContext};
use crate::repository::{count_url, select_url};

#[derive(Clone, Copy, Debug)]
pub struct Query;

#[graphql_object]
#[graphql_object(context = ServerContext)]
impl Query {
    async fn meetup_url_list (
        #[graphql(context)] _server_context: &ServerContext,
        filter: MeetupUrlFilter) -> Vec<MeetupUrl> {
        log!(Level::Info, "Received request query: {:?}", filter);

        let client = connect_db().await;
        let result = select_url(&client, filter).await;

        result.unwrap_or(Vec::new())
    }

    async fn meetup_url_count (
        #[graphql(context)] _server_context: &ServerContext,
        filter: MeetupUrlFilter) -> MeetupUrlCount {
        log!(Level::Info, "Received request count: {:?}", filter);
        
        let client = connect_db().await;
        let result = count_url(&client, filter).await;
        
        MeetupUrlCount {count : result.unwrap_or(0) as i32 }
    }
}