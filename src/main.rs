mod components;
mod models;

use leptos::prelude::*;
use crate::components::app::App;
use thaw::*;

fn main() {
    mount_to_body(|| view! {
        <ConfigProvider>
            <App/>
        </ConfigProvider>
    })
}