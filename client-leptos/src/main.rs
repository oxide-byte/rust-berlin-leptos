#![feature(int_roundings)]

mod component;
mod model;
mod graphql;
mod auth_config;

use crate::component::{App, GlobalState, KeyCloakCatcher};
use leptos::prelude::*;
use reactive_stores::Store;
use thaw::*;

fn main() {
    // Initialize GlobalState
    let state = Store::new(GlobalState::default());

    mount_to_body(move || {
        provide_context(state.clone());

        view! {
            <ConfigProvider>
                <KeyCloakCatcher/>
                <App/>
            </ConfigProvider>
        }
    })
}