use crate::config::connect_db;
use crate::graphql::{MeetupUrl, ServerContext, UpsertMeetupUrl};
use crate::repository::{delete_by_uri_uuid, insert_meetup_url, update_meetup_url};
use crate::service::init_database;
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

    async fn insert_meetup_url(
        #[graphql(context)] _server_context: &ServerContext,
        meetup_url: UpsertMeetupUrl) -> MeetupUrl {
        log!(Level::Info, "Received Insert request: {:?}", meetup_url);

        let client = connect_db().await;

        let result = insert_meetup_url(&client, meetup_url).await;

        result.unwrap()
    }

    async fn update_meetup_url(
        #[graphql(context)] _server_context: &ServerContext,
        meetup_url: UpsertMeetupUrl) -> MeetupUrl {
        log!(Level::Info, "Received Update request: {:?}", meetup_url);

        let client = connect_db().await;

        let result = update_meetup_url(&client, meetup_url).await;

        result.unwrap()
    }

    async fn init_database(
        #[graphql(context)] _server_context: &ServerContext) -> i32 {
        log!(Level::Info, "Init Database");

        let client = connect_db().await;

        init_database(&client).await;

        1
    }
}