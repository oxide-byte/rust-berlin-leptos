use crate::config::connect_db;
use crate::graphql::{MeetupUrl, ServerContext, UpsertMeetupUrl};
use crate::repository::{delete_by_uri_uuid, insert_meetup_url, update_meetup_url};
use crate::service::init_database;
use async_graphql::{Context, Object};
use tracing::log::{log, Level};
use crate::auth::Claims;

#[derive(Clone, Copy, Debug)]
pub struct Mutation;

#[Object]
impl Mutation {
    async fn delete_meetup_url(&self, ctx: &Context<'_>, id: String) -> i32 {
        log!(Level::Info, "Received Delete request: {:?}", id);

        let _server_context = ctx.data_unchecked::<ServerContext>();

        let client = connect_db().await;

        let result = delete_by_uri_uuid(&client, id).await;

        if result.is_ok() {
            1
        } else {
            0
        }
    }

    async fn insert_meetup_url(&self, ctx: &Context<'_>, meetup_url: UpsertMeetupUrl) -> MeetupUrl {
        log!(Level::Info, "Received Insert request: {:?}", meetup_url);

        let _server_context = ctx.data_unchecked::<ServerContext>();

        let client = connect_db().await;

        let result = insert_meetup_url(&client, meetup_url).await;

        result.unwrap()
    }

    async fn update_meetup_url(&self, ctx: &Context<'_>, meetup_url: UpsertMeetupUrl) -> MeetupUrl {
        log!(Level::Info, "Received Update request: {:?}", meetup_url);

        let _server_context = ctx.data_unchecked::<ServerContext>();

        let client = connect_db().await;

        let result = update_meetup_url(&client, meetup_url).await;

        result.unwrap()
    }

    async fn init_database(&self, ctx: &Context<'_>) -> i32 {
        log!(Level::Info, "Init Database");

        let is_admin = ctx.data::<Claims>()
            .map(|c| c.has_role("hackandlearn-client", "ROLE_HNL_ADMIN"))
            .unwrap_or(false);

        if is_admin {
            let client = connect_db().await;
            init_database(&client).await;
            return 1;
        } else {
            log!(Level::Warn, "NOT AUTHORIZED !!!");
            return 0;
        }
    }
}