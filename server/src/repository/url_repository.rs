use crate::graphql::MeetupUrlFilter;
use crate::graphql::{MeetupUrl as GraphMeetupUrl, MeetupUrl, UpsertMeetupUrl};
use crate::model::MeetupUrl as DbMeetupUrl;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::{Id, Strand, Thing};
use surrealdb::{Error, Surreal};
use tracing::log::{log, Level};

#[derive(Debug, Serialize, Deserialize)]
struct Record {
    id: Thing,
    uri_uuid: Strand,
    url: Strand,
    scheme: Strand,
    host: Strand,
    path: Strand,
    live_status: Strand,
    title: Strand,
    auto_descr: Strand,
    man_descr: Strand,
    crea_user: Strand,
    crea_time: Strand,
    modi_user: Strand,
    modi_time: Strand,
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateRecord {
    url: String,
    host: String,
    path: String,
    title: String,
    auto_descr: String,
    man_descr: String,
    modi_user: String,
    modi_time: String,
}

pub async fn insert_init_meetup_url(url: DbMeetupUrl, client: &Surreal<Client>) -> Result<(), Error> {
    let _created: Vec<Record> = client
        .insert("url")
        .content(url)
        .await.expect("Error adding Record");

    Ok(())
}

pub async fn count_url(client: &Surreal<Client>, filter: MeetupUrlFilter) -> Result<i32, Error> {
    let cond = query_builder(filter, false);

    let query = format!("SELECT count() FROM url WHERE 1 = 1 {} GROUP BY count", cond);
    log!(Level::Info, "Query: {}", query);

    let count: Option<i32> = client
        .query(query).await?.take("count")?;

    Ok(count.unwrap_or(0))
}

pub async fn delete_by_uri_uuid(client: &Surreal<Client>, uuid_id: String) -> Result<(), Error> {
    let id = find_by_uri_uuid(client, uuid_id).await?;

    let _result: Option<Record> = client
        .delete(("url", id.to_string()))
        .await?;

    Ok(())
}

pub async fn insert_meetup_url(client: &Surreal<Client>, data: UpsertMeetupUrl) -> Result<MeetupUrl, Error> {
    let now = Utc::now().to_string();

    let meetup_url = DbMeetupUrl {
        uri_uuid: format!("UUID_{}", now),
        url: data.url.clone(),
        scheme: "SCHEMA".to_string(),
        host: data.host.clone(),
        path: "PATH".to_string(),
        live_status: "OK".to_string(),
        title: data.title.clone(),
        auto_descr: data.auto_descr.clone(),
        man_descr: data.auto_descr.clone(),
        crea_user: "API".to_string(),
        crea_time: now.clone(),
        modi_user: "API".to_string(),
        modi_time: now.clone(),
    };

    let created: Vec<Record> = client
        .insert("url")
        .content(meetup_url)
        .await.expect("Error adding Record");

    let rtn = created.get(0)
        .map(map_record_to_graph_meetup_url())
        .unwrap();

    Ok(rtn)
}

pub async fn update_meetup_url(client: &Surreal<Client>, data: UpsertMeetupUrl) -> Result<MeetupUrl, Error> {
    let now = Utc::now().to_string();

    let query = format!("SELECT * FROM url WHERE uri_uuid = '{}'", data.uri_uuid.clone().unwrap());

    let records: Vec<Record> = client.query(query).await?.take(0)?;

    let record = records.first().unwrap();

    let id = record.id.id.to_raw();

    let update = UpdateRecord {
        url: data.url.clone(),
        host: data.host.clone(),
        path: data.url.clone(),
        title: data.title.clone(),
        auto_descr: data.auto_descr.clone(),
        man_descr: data.auto_descr.clone(),
        modi_user: "API_UPDATE".to_string(),
        modi_time: now,
    };

    println!("Update Record {:?} [{:?}]", id, update);
    let updated: Option<Record> = client
        .update(("url", id))
        .merge(update)
        .await
        .map_err(|e| {
            eprintln!("Error Updating Record: {:?}", e);
            e
        })?;

    let rtn = updated
        .as_ref()
        .map(map_record_to_graph_meetup_url())
        .unwrap();

    Ok(rtn)
}

pub async fn find_by_uri_uuid(client: &Surreal<Client>, uuid_id: String) -> Result<Id, Error> {
    let query = format!("SELECT * FROM url WHERE uri_uuid = '{}'", uuid_id);

    let records: Vec<Record> = client.query(query).await?.take(0)?;

    let record = records.first().unwrap();

    Ok(record.id.id.clone())
}

pub async fn select_url(client: &Surreal<Client>, filter: MeetupUrlFilter) -> Result<Vec<GraphMeetupUrl>, Error> {
    let cond = query_builder(filter, true);

    let query = format!("SELECT * FROM url WHERE 1 = 1 {}", cond);
    log!(Level::Info, "Query: {}", query);

    let records: Vec<Record> = client.query(query).await?.take(0)?;


    let urls = records.iter()
        .map(map_record_to_graph_meetup_url())
        .collect::<Vec<GraphMeetupUrl>>();

    Ok(urls)
}

fn map_record_to_graph_meetup_url() -> fn(&Record) -> MeetupUrl {
    |x| GraphMeetupUrl {
        uri_uuid: x.uri_uuid.clone().as_string(),
        url: x.url.clone().as_string(),
        scheme: x.scheme.clone().as_string(),
        host: x.host.clone().as_string(),
        path: x.path.clone().as_string(),
        live_status: x.live_status.clone().as_string(),
        title: x.title.clone().as_string(),
        auto_descr: x.auto_descr.clone().as_string(),
        man_descr: x.man_descr.clone().as_string(),
        crea_user: x.crea_user.clone().as_string(),
        crea_time: x.crea_time.clone().as_string(),
        modi_user: x.modi_user.clone().as_string(),
        modi_time: x.modi_time.clone().as_string(),
    }
}

fn query_builder(filter: MeetupUrlFilter, pagination: bool) -> String {
    let mut cond = "".to_string();

    if let Some(description) = filter.description {
        cond = format!(" {} AND string::matches(auto_descr,'{}') ", cond, description)
    }

    if let Some(domain) = filter.domain {
        cond = format!(" {} AND string::matches(host,'{}') ", cond, domain)
    }

    if let Some(url) = filter.url {
        cond = format!(" {} AND string::matches(url,'{}') ", cond, url)
    }

    if let Some(title) = filter.title {
        cond = format!(" {} AND string::matches(title,'{}') ", cond, title)
    }

    if pagination {
        if let Some(page) = filter.pagination {
            if let Some(current) = page.current {
                cond = format!(" {} START AT {}", cond, current)
            }
            if let Some(size) = page.size {
                cond = format!(" {} LIMIT BY {}", cond, size)
            }
        }
    }
    cond
}