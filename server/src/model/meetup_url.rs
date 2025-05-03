use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MeetupUrl {
    pub uri_uuid: String,
    pub url: String,
    pub scheme: String,
    pub host: String,
    pub path: String,
    pub live_status: String,
    pub title: String,
    pub auto_descr: String,
    pub man_descr: String,
    pub crea_user: String,
    pub crea_time: String,
    pub modi_user: String,
    pub modi_time: String
}