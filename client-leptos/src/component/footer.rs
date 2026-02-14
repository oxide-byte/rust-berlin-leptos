use crate::component::{ClockComponent, GlobalState};
use leptos::prelude::*;
use reactive_stores::Store;
use crate::component::keycloak_catcher::GlobalStateStoreFields;

#[component]
pub fn Footer() -> impl IntoView {
    let state = expect_context::<Store<GlobalState>>();
    
    view! {
        <div>
            <p class="text-center">"©Rust Hack&Learn Meetup Berlin, 2024, Version 20240912.001"</p>
                    <Show
                when=move || state.is_authenticated().get()
            >
            <p class="text-center"><ClockComponent></ClockComponent></p>
            </Show>
        </div>
    }
}