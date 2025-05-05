use crate::component::banner::Banner;
use crate::component::event_table::EventTable;
use crate::component::footer::Footer;
use crate::component::navigation_bar::NavigationBar;
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