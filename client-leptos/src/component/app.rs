use crate::component::{Banner, EventTable, Footer, GlobalState, NavigationBar};
use leptos::prelude::*;
use reactive_stores::Store;
use crate::component::keycloak_catcher::GlobalStateStoreFields;

#[component]
pub fn App() -> impl IntoView {
    let state = expect_context::<Store<GlobalState>>();

    view! {
        <NavigationBar/>
        <div class="ml-20 mr-20">
            <Banner/>
            <hr/>
            <Show
                when=move || state.is_authenticated().get()
                fallback=|| view! { <p> Please Login </p> }
            >
                <EventTable/>
            </Show>
            <hr/>
            <Footer/>
        </div>
    }
}