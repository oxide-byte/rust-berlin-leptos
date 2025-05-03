use juniper::{Context, GraphQLInputObject, GraphQLObject};

#[derive(Clone, Default)]
pub struct ServerContext {
}

impl Context for ServerContext {}

#[derive(GraphQLInputObject, Debug)]
pub struct MeetupUrlFilter {
    pub domain: Option<String>,
    pub title: Option<String>,
    pub url: Option<String>,
    pub description: Option<String>,
}

#[derive(GraphQLObject)]
pub struct MeetupUrl {
    pub uri_uuid: String,
    pub url: String,
    pub scheme: String,
    pub host: String,
    pub path: String,
    pub live_status: String,
    pub title: String,
    pub auto_descr: String,
    pub man_descr: String,
    pub crea_user: String,
    pub crea_time: String,
    pub modi_user: String,
    pub modi_time: String
}

#[derive(GraphQLObject)]
pub struct MeetupUrlCount {
    pub count: i32
}

#[derive(GraphQLObject)]
pub struct ClockBox {
    pub clock: String
}