use crate::components::banner::Banner;
use crate::components::event_table::EventTable;
use crate::components::footer::Footer;
use crate::components::navigation_bar::NavigationBar;
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <NavigationBar/>
        <div class="ml-20 mr-20">
            <Banner/>
            <hr/>
            <EventTable/>
            <hr/>
            <Footer/>
        </div>
    }
}