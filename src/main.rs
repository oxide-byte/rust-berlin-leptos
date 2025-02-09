mod components;
mod models;

use leptos::prelude::*;
use crate::components::app::App;

fn main() {
    mount_to_body(|| view! { <App/> })
}