mod components;
mod models;
mod graphql;

use crate::components::app::App;
use leptos::prelude::*;
use thaw::*;

fn main() {
    mount_to_body(|| view! {
        <ConfigProvider>
            <App/>
        </ConfigProvider>
    })
}