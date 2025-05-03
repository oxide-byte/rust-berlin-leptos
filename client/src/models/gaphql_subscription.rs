// {"type":"next","id":"fb087db7-0baa-48f7-8c47-ff24575465a7","payload":{"data":{"clock":{"clock":"2025-05-02 20:02:55.020169 UTC"}}}}
// https://transform.tools/json-to-rust-serde

use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClockSubscriptionResponse {
    #[serde(rename = "type")]
    pub type_field: String,
    pub id: String,
    pub payload: Payload,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Payload {
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub clock: Clock,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Clock {
    pub clock: String,
}