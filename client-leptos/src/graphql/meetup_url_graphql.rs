use crate::graphql::meetup_url_graphql::meetup_url_insert_mutation::UpsertMeetupUrl as InsertMeetupUrl;
use crate::graphql::meetup_url_graphql::meetup_url_query::MeetupUrlQueryMeetupUrlListResult;
use crate::graphql::meetup_url_graphql::meetup_url_update_mutation::UpsertMeetupUrl as UpdateMeetupUrl;
use crate::model::Event;
use crate::model::Filter;
use crate::model::MeetupUrlEdit;
use graphql_client::{reqwest::post_graphql, GraphQLQuery};
use ::reqwest::Client;

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

pub async fn fetch_meetup_url_data(filter: Filter) -> (Vec<Event>, i64) {
    let client = Client::builder().build().unwrap();
    let endpoint = "http://localhost:8080/graphql";

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

    let variables = meetup_url_query::Variables {
        filter,
    };

    // Await the GraphQL request
    let response = post_graphql::<MeetupUrlQuery, _>(&client, endpoint, variables)
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

    let variables = meetup_url_delete_mutation::Variables {
        id: uuid,
    };

    // Await the GraphQL request
    let _response = post_graphql::<MeetupUrlDeleteMutation, _>(&client, endpoint, variables)
        .await
        .expect("Failed to execute GraphQL query");
}

pub async fn insert_meetup_event(item: MeetupUrlEdit) {
    let client = Client::builder().build().unwrap();
    let endpoint = "http://localhost:8080/graphql";

    let variables = meetup_url_insert_mutation::Variables {
        upsert_meetup_url: InsertMeetupUrl {
            uri_uuid: None,
            url: item.url.unwrap(),
            host: item.domain.unwrap(),
            title: item.title.unwrap(),
            auto_descr: item.description.unwrap(),
        }
    };

    // Await the GraphQL request
    let _response = post_graphql::<MeetupUrlInsertMutation, _>(&client, endpoint, variables)
        .await
        .expect("Failed to execute GraphQL query");
}

pub async fn update_meetup_event(item: MeetupUrlEdit) {
    let client = Client::builder().build().unwrap();
    let endpoint = "http://localhost:8080/graphql";

    let variables = meetup_url_update_mutation::Variables {
        upsert_meetup_url: UpdateMeetupUrl {
            uri_uuid: Some(item.uri_uuid.unwrap()),
            url: item.url.unwrap(),
            host: item.domain.unwrap(),
            title: item.title.unwrap(),
            auto_descr: item.description.unwrap(),
        }
    };

    // Await the GraphQL request
    let _response = post_graphql::<MeetupUrlUpdateMutation, _>(&client, endpoint, variables)
        .await
        .expect("Failed to execute GraphQL query");
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