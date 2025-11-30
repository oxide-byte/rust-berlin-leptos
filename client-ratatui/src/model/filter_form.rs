use super::FilterGraphql;

#[derive(Clone, Debug, Default)]
pub struct FilterForm {
    pub title_input: String,
    pub domain_input: String,
    pub url_input: String,
    pub description_input: String,
    pub active_index: usize, // 0=title,1=domain,2=url,3=description
}

impl FilterForm {
    pub fn set_from_filter(&mut self, filter: &FilterGraphql) {
        self.title_input = filter.title.clone().unwrap_or_default();
        self.domain_input = filter.domain.clone().unwrap_or_default();
        self.url_input = filter.url.clone().unwrap_or_default();
        self.description_input = filter.description.clone().unwrap_or_default();
        self.active_index = 0;
    }

    pub fn to_filter(&self) -> FilterGraphql {
        let mut filter = FilterGraphql::default();
        let t = self.title_input.trim();
        let d = self.domain_input.trim();
        let u = self.url_input.trim();
        let desc = self.description_input.trim();
        if !t.is_empty() { filter.title = Some(t.to_string()); }
        if !d.is_empty() { filter.domain = Some(d.to_string()); }
        if !u.is_empty() { filter.url = Some(u.to_string()); }
        if !desc.is_empty() { filter.description = Some(desc.to_string()); }
        filter
    }

    pub fn cycle_next(&mut self) {
        self.active_index = (self.active_index + 1) % 4;
    }

    pub fn cycle_prev(&mut self) {
        self.active_index = if self.active_index == 0 { 3 } else { self.active_index - 1 };
    }

    pub fn push_active(&mut self, c: char) {
        match self.active_index {
            0 => self.title_input.push(c),
            1 => self.domain_input.push(c),
            2 => self.url_input.push(c),
            3 => self.description_input.push(c),
            _ => {}
        }
    }

    pub fn pop_active(&mut self) {
        match self.active_index {
            0 => { self.title_input.pop(); },
            1 => { self.domain_input.pop(); },
            2 => { self.url_input.pop(); },
            3 => { self.description_input.pop(); },
            _ => {}
        }
    }
}