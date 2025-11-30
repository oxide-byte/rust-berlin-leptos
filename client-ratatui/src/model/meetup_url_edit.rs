#[derive(Clone, Debug, Default)]
pub struct MeetupUrlEdit {
    pub uri_uuid: Option<String>,
    pub title: Option<String>,
    pub domain: Option<String>,
    pub url: Option<String>,
    pub description: Option<String>,
}