use crate::graphql::subscription_graphql::ClockSubscriptionResponse;
use futures::{SinkExt, StreamExt};
use leptos::prelude::*;
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;
use ws_stream_wasm::*;

// Leptos component
#[component]
pub fn ClockComponent() -> impl IntoView {
    let (read_clock, write_clock) = signal(String::from("test"));

    // Start the GraphQL subscription
    spawn_local(async move {

        // Create a WebSocket connection
        let ws_url = "ws://localhost:8080/subscriptions";
        let (_ws, mut wsio) = match WsMeta::connect(ws_url, None).await {
            Ok(ws) => ws,
            Err(e) => {
                write_clock.set(format!("WS connect error: {e}"));
                return;
            }
        };

        // Init connection
        let conn_init = serde_json::to_string(&serde_json::json!({
                "type": "connection_init",
                "payload": {}
            })).unwrap();
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