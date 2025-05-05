mod component;
mod model;
mod graphql;

use crate::component::app::App;
use leptos::prelude::*;
use thaw::*;

fn main() {
    mount_to_body(|| view! {
        <ConfigProvider>
            <App/>
        </ConfigProvider>
    })
}