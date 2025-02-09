#[derive(Clone, Debug)]
pub struct Event {
    pub id:usize,
    pub title:String,
    pub domain: String,
    pub url:String,
    pub description: String,
}

impl Event {
    fn new(id:usize, title:String, domain: String, url: String, description: String) -> Self {
        Event {id, title, domain, url, description}
    }
}

pub fn generate_demo(count: usize) -> Vec<Event> {
    let mut rtn:Vec<Event> = Vec::new();
    for i in 0 ..count {
        rtn.push(Event::new(
            i,
            format!("Title ({})",i),
            format!("Domain ({})",i),
            format!("http://URL_{}",i),
            format!("Description ({})", i)
        ));
    }
    rtn
}