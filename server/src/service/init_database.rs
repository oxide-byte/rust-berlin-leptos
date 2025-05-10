use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use crate::repository::insert_init_meetup_url;
use crate::service::import_data;

pub async fn init_database(client: &Surreal<Client>) {
    let data = import_data();

    for url in data {
        let _ = insert_init_meetup_url(url, &client).await;
    }
}

#[cfg(test)]
mod tests {
    use crate::config::connect_db;
    use crate::service::init_database::init_database;

    #[tokio::test]
    async fn test_init_database() {
        let client = connect_db().await;
        init_database(&client).await;
    }
}