use crate::config::connect_db;
use crate::repository::insert_url;
use crate::service::import_data;

pub async fn init_database()  {
    
    let data = import_data();
    let client = connect_db().await;

    for url in data {
        let _ = insert_url(url, &client).await;
    }
}

#[cfg(test)]
mod tests {
    use crate::service::init_database::init_database;

    #[tokio::test]
    async fn test_init_database() {
        init_database().await;
    }
}