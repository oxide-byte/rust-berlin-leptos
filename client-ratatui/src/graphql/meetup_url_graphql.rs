use crate::graphql::meetup_url_graphql::meetup_url_query::MeetupUrlQueryMeetupUrlListResult;
use crate::model::{Event, FilterGraphql};
use graphql_client::{reqwest::post_graphql, GraphQLQuery};
use ::reqwest::Client;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/meetup_url.graphql",
)]
pub struct MeetupUrlQuery;

const ENDPOINT: &str = "http://localhost:8080/graphql";

pub async fn fetch_meetup_url_data(filter: &FilterGraphql) -> (Vec<Event>, i64) {
    let client = match Client::builder().build() {
        Ok(c) => c,
        Err(_e) => {
            return (Vec::new(), 0);
        }
    };
    fetch_meetup_url_data_with(&client, filter).await
}

pub async fn fetch_meetup_url_data_with(client: &Client, filter: &FilterGraphql) -> (Vec<Event>, i64) {
    let page = match (filter.page, filter.size) {
        (Some(current), Some(size)) => Some(meetup_url_query::Pagination { current, size }),
        _ => None,
    };

    let filter = meetup_url_query::MeetupUrlFilter {
        domain: filter.domain.clone(),
        title: filter.title.clone(),
        url: filter.url.clone(),
        description: filter.description.clone(),
        pagination: page,
        sort: None,
    };

    let variables = meetup_url_query::Variables {
        filter,
    };

    let response = match post_graphql::<MeetupUrlQuery, _>(client, ENDPOINT, variables).await {
        Ok(resp) => resp,
        Err(_e) => {
            return (Vec::new(), 0);
        }
    };

    if let Some(data) = response.data {
        (
            meetup_url_to_event(data.meetup_url_list.result),
            data.meetup_url_count.count as i64,
        )
    } else {
        (Vec::new(), 0)
    }
}

fn meetup_url_to_event(data: Vec<MeetupUrlQueryMeetupUrlListResult>) -> Vec<Event> {
    data.into_iter()
        .map(|e| Event {
            title: e.title,
            domain: e.host,
            url: e.url,
            description: e.auto_descr,
        })
        .collect()
}