use juniper::graphql_object;
use tracing::log::{Level, log};
use crate::graphql::{MeetupUrlFilter, MeetupUrl, ServerContext};


#[derive(Clone, Copy, Debug)]
pub struct Mutation;

#[graphql_object]
#[graphql_object(context = ServerContext)]
impl Mutation {
    
    fn add_meetup_url(
        #[graphql(context)] _server_context: &ServerContext,
        filter: MeetupUrlFilter) -> MeetupUrl {
        log!(Level::Info, "Received request: {:?}", filter);

        MeetupUrl{
            uri_uuid: "".to_string(),
            url: "".to_string(),
            scheme: "".to_string(),
            host: "".to_string(),
            path: "".to_string(),
            live_status: "".to_string(),
            title: "".to_string(),
            auto_descr: "".to_string(),
            man_descr: "".to_string(),
            crea_user: "".to_string(),
            crea_time: "".to_string(),
            modi_user: "".to_string(),
            modi_time: "".to_string(),
        }
    }

    fn modify_meetup_url(
        #[graphql(context)] _server_context: &ServerContext,
        filter: MeetupUrlFilter) -> MeetupUrl {
        log!(Level::Info, "Received request: {:?}", filter);

        MeetupUrl{
            uri_uuid: "".to_string(),
            url: "".to_string(),
            scheme: "".to_string(),
            host: "".to_string(),
            path: "".to_string(),
            live_status: "".to_string(),
            title: "".to_string(),
            auto_descr: "".to_string(),
            man_descr: "".to_string(),
            crea_user: "".to_string(),
            crea_time: "".to_string(),
            modi_user: "".to_string(),
            modi_time: "".to_string(),
        }
    }

    fn delete_meetup_url(
        #[graphql(context)] _server_context: &ServerContext,
        filter: MeetupUrlFilter) -> MeetupUrl {
        log!(Level::Info, "Received request: {:?}", filter);

        MeetupUrl{
            uri_uuid: "".to_string(),
            url: "".to_string(),
            scheme: "".to_string(),
            host: "".to_string(),
            path: "".to_string(),
            live_status: "".to_string(),
            title: "".to_string(),
            auto_descr: "".to_string(),
            man_descr: "".to_string(),
            crea_user: "".to_string(),
            crea_time: "".to_string(),
            modi_user: "".to_string(),
            modi_time: "".to_string(),
        }
    }
}