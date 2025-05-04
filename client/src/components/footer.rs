use crate::components::clock_component::ClockComponent;
use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <div>
            <p class="text-center">"©Rust Hack&Learn Meetup Berlin, 2024, Version 20240912.001"</p>
            <p class="text-center"><ClockComponent></ClockComponent></p>
        </div>
    }
}