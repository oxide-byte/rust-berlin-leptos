#[derive(Clone, Debug, Default, PartialOrd, PartialEq)]
pub enum MeetupUrlEditMode {
    #[default]
    INSERT, 
    UPDATE
}
#[derive(Clone, Debug, Default)]
pub struct MeetupUrlEdit {
    pub mode: MeetupUrlEditMode,
    pub title: Option<String>,
    pub domain: Option<String>,
    pub url: Option<String>,
    pub description: Option<String>,
}