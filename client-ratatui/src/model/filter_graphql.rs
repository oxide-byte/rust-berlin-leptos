#[derive(Clone, Debug, Default)]
pub struct FilterGraphql {
    pub page: Option<i64>,
    pub size: Option<i64>,
    pub title: Option<String>,
    pub domain: Option<String>,
    pub url: Option<String>,
    pub description: Option<String>,
}