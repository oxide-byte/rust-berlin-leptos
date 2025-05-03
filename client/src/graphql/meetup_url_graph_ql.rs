use graphql_client::{reqwest::post_graphql, GraphQLQuery};
use ::reqwest::Client;
use crate::graphql::meetup_url_graph_ql::meet_up_url_query::MeetUpUrlQueryMeetupUrlList;
use crate::models::event::Event;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/meetup_url.graphql",
)]
pub struct MeetUpUrlQuery;

pub async fn fetch_meetup_url_data() -> Vec<Event> {
    
    let client = Client::builder().build().unwrap();
    let endpoint = "http://localhost:8080/graphql";

    let filter = meet_up_url_query::MeetupUrlFilter{
        domain: None,
        title: None,
        url: None,
        description: None
    };

    let variables = meet_up_url_query::Variables {
        filter,
    };

    // Await the GraphQL request
    let response = post_graphql::<MeetUpUrlQuery, _>(&client, endpoint, variables)
        .await
        .expect("Failed to execute GraphQL query");

    response
        .data
        .map(|data| data.meetup_url_list)
        .map(|data| meetup_url_to_event(data))
        .unwrap_or_default()
    
}

fn meetup_url_to_event(data: Vec<MeetUpUrlQueryMeetupUrlList>) -> Vec<Event> {
    data.iter()
        .map(|e| Event{
            id:e.uri_uuid.clone(),
            title: e.title.clone(),
            domain: e.host.clone(),
            url: e.url.clone(),
            description: e.auto_descr.clone(),
        })
        .collect()
}