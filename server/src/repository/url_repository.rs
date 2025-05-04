use crate::graphql::MeetupUrl as GraphMeetupUrl;
use crate::graphql::MeetupUrlFilter;
use crate::model::MeetupUrl as DbMeetupUrl;
use serde::Deserialize;
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::{Strand, Thing};
use surrealdb::{Error, Surreal};
use tracing::log::{log, Level};

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
    #[allow(dead_code)]
    uri_uuid: Strand,
    #[allow(dead_code)]
    url: Strand,
    #[allow(dead_code)]
    scheme: Strand,
    #[allow(dead_code)]
    host: Strand,
    #[allow(dead_code)]
    path: Strand,
    #[allow(dead_code)]
    live_status: Strand,
    #[allow(dead_code)]
    title: Strand,
    #[allow(dead_code)]
    auto_descr: Strand,
    #[allow(dead_code)]
    man_descr: Strand,
    #[allow(dead_code)]
    crea_user: Strand,
    #[allow(dead_code)]
    crea_time: Strand,
    #[allow(dead_code)]
    modi_user: Strand,
    #[allow(dead_code)]
    modi_time: Strand,
}

pub async fn insert_url(url: DbMeetupUrl, client: &Surreal<Client>) -> Result<(), Error> {
    let _created: Vec<Record> = client
        .insert("url")
        .content(url)
        .await.expect("Error adding Record");

    Ok(())
}

pub async fn count_url(client: &Surreal<Client>, filter: MeetupUrlFilter) -> Result<i32, Error> {
    let cond = query_builder(filter);

    let query = format!("SELECT count() FROM url WHERE 1 = 1 {} GROUP BY count", cond);
    log!(Level::Info, "Query: {}", query);

    let count: Option<i32> = client
        .query(query).await?.take("count")?;

    Ok(count.unwrap_or(0))
}

pub async fn select_url(client: &Surreal<Client>, filter: MeetupUrlFilter) -> Result<Vec<GraphMeetupUrl>, Error> {
    let cond = query_builder(filter);

    let query = format!("SELECT * FROM url WHERE 1 = 1 {}", cond);
    log!(Level::Info, "Query: {}", query);

    let records: Vec<Record> = client.query(query).await?.take(0)?;


    let urls = records.iter().map(|x| GraphMeetupUrl {
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
    }).collect::<Vec<GraphMeetupUrl>>();

    Ok(urls)
}

fn query_builder(filter: MeetupUrlFilter) -> String {
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

    if let Some(page) = filter.pagination {
        if let Some(current) = page.current {
            cond = format!(" {} START AT {}", cond, current)
        }
        if let Some(size) = page.size {
            cond = format!(" {} LIMIT BY {}", cond, size)
        }
    }
    cond
}
