use crate::component::GlobalState;
use crate::graphql::ClockSubscriptionResponse;
use futures::{SinkExt, StreamExt};
use leptos::logging::log;
use leptos::prelude::*;
use reactive_stores::Store;
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;
use ws_stream_wasm::*;
use crate::component::keycloak_catcher::GlobalStateStoreFields;

// Leptos component
#[component]
pub fn ClockComponent() -> impl IntoView {
    let (read_clock, write_clock) = signal(String::from("test"));
    
    // Get auth token from GlobalState if available - retrieve it outside spawn_local
    let token = use_context::<Store<GlobalState>>()
        .and_then(|state| state.token().get());
    log!("[ClockComponent] token: {:?}", token);
    // Start the GraphQL subscription
    spawn_local(async move {
        if token.is_some() {
            log!("[GraphQL Subscription] Using authenticated connection");
        } else {
            log!("[GraphQL Subscription] Using unauthenticated connection");
        }

        // Create a WebSocket connection
        // async-graphql uses the `graphql-transport-ws` subprotocol for subscriptions
        let ws_url = crate::auth_config::GRAPHQL_WS_ENDPOINT;
        let (_ws, mut wsio) = match WsMeta::connect(ws_url, Some(vec!["graphql-transport-ws"])) .await {
            Ok(ws) => ws,
            Err(e) => {
                log!("WS connect error: {e}");
                write_clock.set(format!("WS connect error: {e}"));
                return;
            }
        };

        // Init connection with optional authorization token
        let payload = if let Some(token) = token {
            serde_json::json!({
                "Authorization": format!("Bearer {}", token)
            })
        } else {
            serde_json::json!({})
        };

        let conn_init = serde_json::to_string(&serde_json::json!({
            "type": "connection_init",
            "payload": payload
        })).unwrap();

        log!("[GraphQL Subscription] Connection init: {}", conn_init);
        wsio.send(WsMessage::Text(conn_init)).await.ok();
        wsio.next().await;

        // Send start subscription
        let id = Uuid::new_v4().to_string();
        let start_msg = serde_json::to_string(&serde_json::json!({
            "id":id,
            "type":"subscribe",
            "payload":{"query":"subscription tt {\n  clock {\n    clock\n  }\n}",
            "operationName":"tt"}
            })).unwrap();
        wsio.send(WsMessage::Text(start_msg)).await.ok();
        write_clock.set("demo".to_string());

        // Listen for messages
        while let Some(msg) = wsio.next().await {
            if let WsMessage::Text(data) = msg {
                let v: ClockSubscriptionResponse = match serde_json::from_str(&*data) {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                write_clock.set(format!("bb {:?}", &v));

                if v.type_field == "next".to_string() && v.id == id {
                    write_clock.set(v.payload.data.clock.clock);
                }
            }
        }
    });

    // Render the component
    view! {
        <p>{ move || read_clock.get() }</p>
    }
}