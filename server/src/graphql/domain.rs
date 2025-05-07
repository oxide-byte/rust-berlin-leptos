use juniper::{Context, GraphQLEnum, GraphQLInputObject, GraphQLObject};

#[derive(Debug, Clone, Default)]
pub struct ServerContext {}

impl Context for ServerContext {}

#[derive(GraphQLEnum, Debug, Copy, Clone, Eq, PartialEq)]
pub enum MeetupUrlSort {
    DOMAIN,
    TITLE,
    URL,
    DESCRIPTION,
}

#[derive(GraphQLInputObject, Debug, Clone)]
pub struct MeetupUrlFilter {
    pub domain: Option<String>,
    pub title: Option<String>,
    pub url: Option<String>,
    pub description: Option<String>,
    pub pagination: Option<Pagination>,
    pub sort: Option<MeetupUrlSort>,
}

#[derive(GraphQLInputObject, Debug, Clone)]
pub struct Pagination {
    pub current: Option<i32>,
    pub size: Option<i32>,
}

#[derive(GraphQLObject, Debug, Clone)]
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
    pub modi_time: String,
}

#[derive(GraphQLObject, Debug, Clone)]
pub struct Page {
    pub current: i32,
    pub size: i32,
    pub total: i32,
}

#[derive(GraphQLObject, Debug, Clone)]
pub struct MeetupUrlResponse {
    pub result: Vec<MeetupUrl>,
    pub page: Page,
}

#[derive(GraphQLObject, Debug, Clone)]
pub struct MeetupUrlCount {
    pub count: i32,
}

#[derive(GraphQLObject)]
pub struct ClockBox {
    pub clock: String,
}