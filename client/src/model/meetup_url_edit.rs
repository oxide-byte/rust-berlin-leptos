#[derive(Clone, Debug, Default)]
pub struct MeetupUrlEdit {
    pub title: Option<String>,
    pub domain: Option<String>,
    pub url: Option<String>,
    pub description: Option<String>,
}