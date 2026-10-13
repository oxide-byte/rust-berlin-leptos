use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

pub async fn connect_db() -> Surreal<Client> {

    // Connect to the server
    let db = Surreal::new::<Ws>("localhost:8000")
        .await.expect("cannot connect network");

    // Login User
    db.signin(Root {
        username: "root".to_string(),
        password: "root".to_string(),
    }).await.expect("cannot connect user");

    // SurrealDB 3 no longer creates the namespace / database implicitly
    db.query("DEFINE NAMESPACE IF NOT EXISTS berlin")
        .await.expect("cannot define namespace");
    db.use_ns("berlin")
        .await.expect("cannot select namespace");
    db.query("DEFINE DATABASE IF NOT EXISTS url_inventory")
        .await.expect("cannot define database");

    // Select a specific namespace / database
    db.use_ns("berlin").use_db("url_inventory")
        .await.expect("cannot connect to namespace");

    db
}