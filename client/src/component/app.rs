use crate::component::{Banner, EventTable, Footer, NavigationBar};
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