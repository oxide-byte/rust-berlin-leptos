use crate::graphql::meetup_url_graphql::meet_up_url_query::MeetUpUrlQueryMeetupUrlListResult;
use crate::model::event::Event;
use crate::model::filter::Filter;
use graphql_client::{reqwest::post_graphql, GraphQLQuery};
use ::reqwest::Client;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/meetup_url.graphql",
)]
pub struct MeetUpUrlQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/meetup_url.graphql",
)]
pub struct MeetUpUrlMutation;

pub async fn fetch_meetup_url_data(filter: Filter) -> (Vec<Event>, i64) {
    let client = Client::builder().build().unwrap();
    let endpoint = "http://localhost:8080/graphql";

    let page = if filter.page.is_none() {
        None
    } else {
        Option::from(meet_up_url_query::Pagination {
            current: filter.page.unwrap(),
            size: filter.size.unwrap(),
        })
    };

    let filter = meet_up_url_query::MeetupUrlFilter {
        domain: filter.domain,
        title: filter.title,
        url: filter.url,
        description: filter.description,
        pagination: page,
        sort: None,
    };

    let variables = meet_up_url_query::Variables {
        filter,
    };

    // Await the GraphQL request
    let response = post_graphql::<MeetUpUrlQuery, _>(&client, endpoint, variables)
        .await
        .expect("Failed to execute GraphQL query");

    if let Some(data) = response.data {
        (
            meetup_url_to_event(data.meetup_url_list.result),
            data.meetup_url_count.count as i64,
        )
    } else {
        (Vec::new(), 0)
    }
}

pub async fn delete_meetup_url_by_uuid_id(uuid: String) {
    let client = Client::builder().build().unwrap();
    let endpoint = "http://localhost:8080/graphql";


    let variables = meet_up_url_mutation::Variables {
        id: uuid,
    };

    // Await the GraphQL request
    let _response = post_graphql::<MeetUpUrlMutation, _>(&client, endpoint, variables)
        .await
        .expect("Failed to execute GraphQL query");
}

fn meetup_url_to_event(data: Vec<MeetUpUrlQueryMeetupUrlListResult>) -> Vec<Event> {
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