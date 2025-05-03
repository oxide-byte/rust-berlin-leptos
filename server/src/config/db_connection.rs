use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

pub async fn connect_db() -> Surreal<Client> {

    // Connect to the server
    let db = Surreal::new::<Ws>("localhost:8000")
        .await.expect("cannot connect network");

    // Select a specific namespace / database
    db.use_ns("berlin").use_db("url_inventory")
        .await.expect("cannot connect to namespace");

    // Login User
    db.signin(Root {
        username: "root",
        password: "root",
    }).await.expect("cannot connect user");

    db
}