use crate::config::connect_db;
use crate::graphql::{MeetupUrl, MeetupUrlCount, MeetupUrlFilter, ServerContext};
use crate::repository::{count_url, delete_by_uri_uuid};
use juniper::graphql_object;
use tracing::log::{log, Level};

#[derive(Clone, Copy, Debug)]
pub struct Mutation;

#[graphql_object]
#[graphql_object(context = ServerContext)]
impl Mutation {
    async fn delete_meetup_url(
        #[graphql(context)] _server_context: &ServerContext,
        id: String) -> i32 {
        log!(Level::Info, "Received Delete request: {:?}", id);

        let client = connect_db().await;

        let result = delete_by_uri_uuid(&client, id).await;

        if result.is_ok() {
            1
        } else {
            0
        }
    }
}