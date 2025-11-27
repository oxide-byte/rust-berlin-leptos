#![feature(int_roundings)]

mod component;
mod model;
mod graphql;

use crate::component::App;
use leptos::prelude::*;
use thaw::*;

fn main() {
    mount_to_body(|| view! {
        <ConfigProvider>
            <App/>
        </ConfigProvider>
    })
}