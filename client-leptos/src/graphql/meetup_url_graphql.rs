use crate::graphql::meetup_url_graphql::meetup_url_insert_mutation::UpsertMeetupUrl as InsertMeetupUrl;
use crate::graphql::meetup_url_graphql::meetup_url_query::MeetupUrlQueryMeetupUrlListResult;
use crate::graphql::meetup_url_graphql::meetup_url_update_mutation::UpsertMeetupUrl as UpdateMeetupUrl;
use crate::model::Event;
use crate::model::Filter;
use crate::model::MeetupUrlEdit;
use graphql_client::GraphQLQuery;
use leptos::logging::log;
use reqwest::Client;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/meetup_url.graphql",
)]
pub struct MeetupUrlQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/meetup_url.graphql",
)]
pub struct MeetupUrlDeleteMutation;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/meetup_url.graphql",
)]
pub struct MeetupUrlInsertMutation;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/meetup_url.graphql",
)]
pub struct MeetupUrlUpdateMutation;

use crate::auth_config::GRAPHQL_HTTP_ENDPOINT;

const ENDPOINT: &str = GRAPHQL_HTTP_ENDPOINT;

/// Build a reqwest client with optional Authorization header
fn build_client_with_auth(token: Option<String>) -> Client {
    let mut headers = reqwest::header::HeaderMap::new();

    if let Some(token) = token {
        if let Ok(auth_value) = reqwest::header::HeaderValue::from_str(&format!("Bearer {}", token)) {
            headers.insert(reqwest::header::AUTHORIZATION, auth_value);
            log!("[GraphQL] Adding Authorization header with token");
        }
    } else {
        log!("[GraphQL] No token found, sending unauthenticated request");
    }

    Client::builder()
        .default_headers(headers)
        .build()
        .unwrap()
}

pub async fn fetch_meetup_url_data(filter: Filter, token: Option<String>) -> (Vec<Event>, i64) {
    let client = build_client_with_auth(token);

    let page = if filter.page.is_none() {
        None
    } else {
        Option::from(meetup_url_query::Pagination {
            current: filter.page.unwrap(),
            size: filter.size.unwrap(),
        })
    };

    let filter = meetup_url_query::MeetupUrlFilter {
        domain: filter.domain,
        title: filter.title,
        url: filter.url,
        description: filter.description,
        pagination: page,
        sort: None,
    };

    let variables = meetup_url_query::Variables { filter };

    // Build GraphQL request body and send via reqwest 0.12
    let request_body = MeetupUrlQuery::build_query(variables);
    let http_resp = client
        .post(ENDPOINT)
        .json(&request_body)
        .send()
        .await
        .expect("Failed to send GraphQL HTTP request");

    let response: graphql_client::Response<meetup_url_query::ResponseData> = http_resp
        .json()
        .await
        .expect("Failed to deserialize GraphQL response");

    if let Some(data) = response.data {
        (
            meetup_url_to_event(data.meetup_url_list.result),
            data.meetup_url_count.count as i64,
        )
    } else {
        (Vec::new(), 0)
    }
}

pub async fn delete_meetup_url_by_uuid_id(uuid: String, token: Option<String>) {
    let client = build_client_with_auth(token);

    let variables = meetup_url_delete_mutation::Variables { id: uuid };

    let request_body = MeetupUrlDeleteMutation::build_query(variables);
    let _http_resp = client
        .post(ENDPOINT)
        .json(&request_body)
        .send()
        .await
        .expect("Failed to execute GraphQL delete mutation");
}

pub async fn insert_meetup_event(item: MeetupUrlEdit, token: Option<String>) {
    let client = build_client_with_auth(token);

    let variables = meetup_url_insert_mutation::Variables {
        upsert_meetup_url: InsertMeetupUrl {
            uri_uuid: None,
            url: item.url.unwrap(),
            host: item.domain.unwrap(),
            title: item.title.unwrap(),
            auto_descr: item.description.unwrap(),
        },
    };

    let request_body = MeetupUrlInsertMutation::build_query(variables);
    let _http_resp = client
        .post(ENDPOINT)
        .json(&request_body)
        .send()
        .await
        .expect("Failed to execute GraphQL insert mutation");
}

pub async fn update_meetup_event(item: MeetupUrlEdit, token: Option<String>) {
    let client = build_client_with_auth(token);

    let variables = meetup_url_update_mutation::Variables {
        upsert_meetup_url: UpdateMeetupUrl {
            uri_uuid: Some(item.uri_uuid.unwrap()),
            url: item.url.unwrap(),
            host: item.domain.unwrap(),
            title: item.title.unwrap(),
            auto_descr: item.description.unwrap(),
        },
    };

    let request_body = MeetupUrlUpdateMutation::build_query(variables);
    let _http_resp = client
        .post(ENDPOINT)
        .json(&request_body)
        .send()
        .await
        .expect("Failed to execute GraphQL update mutation");
}

fn meetup_url_to_event(data: Vec<MeetupUrlQueryMeetupUrlListResult>) -> Vec<Event> {
    data.iter()
        .map(|e| Event {
            id: e.uri_uuid.clone(),
            title: e.title.clone(),
            domain: e.host.clone(),
            url: e.url.clone(),
            description: e.auto_descr.clone(),
        })
        .collect()
}