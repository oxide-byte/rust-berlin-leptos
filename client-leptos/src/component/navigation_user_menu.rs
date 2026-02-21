use crate::component::GlobalState;
use keycloak_wasm_auth::{Challenge, LoginParams};
use leptos::prelude::*;
use reactive_stores::{Patch, Store};
use leptos::logging::log;
use thaw::{Button, ButtonAppearance};
use uuid::Uuid;
use crate::component::keycloak_catcher::GlobalStateStoreFields;
use crate::graphql::{init_database};
use crate::component::KeycloakAccessAdmin;

#[component]
pub fn NavigationUserMenu() -> impl IntoView {
    let dropdown_open = RwSignal::new(false);
    let state = expect_context::<Store<GlobalState>>();

    view! {
        <div class="relative ml-3">
            <div>
                <button
                    type="button"
                    class="relative flex rounded-full bg-gray-800 text-sm focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-gray-800 focus:outline-hidden"
                    id="user-menu-button"
                    aria-expanded="false"
                    aria-haspopup="true"
                    on:click=move |_| dropdown_open.update(|v| *v = !*v)
                >
                    <span class="absolute -inset-1.5"></span>
                    <span class="sr-only">Open user menu</span>
                    <img class="size-8 rounded-full" src="./public/avatar.svg" alt="" />
                </button>
            </div>

            <div
                class="absolute right-0 z-10 mt-2 w-48 origin-top-right rounded-md bg-white py-1 shadow-lg ring-1 ring-black/5 focus:outline-hidden"
                class:hidden=move || !dropdown_open.get()
                role="menu"
                aria-orientation="vertical"
                aria-labelledby="user-menu-button"
                tabindex="-1"
            >
                <Show
                    when=move || state.is_authenticated().get()
                    fallback=move || view! { <LoginButton dropdown_open=dropdown_open/> }
                >
                    <UserInfo/>
                    <LogoutButton dropdown_open=dropdown_open/>
                    <InitDatabaseButton dropdown_open=dropdown_open/>
                </Show>
            </div>
        </div>
    }
}

#[component]
pub fn UserInfo() -> impl IntoView {
    let state = expect_context::<Store<GlobalState>>();

    view! {
        <div class="block px-4 py-2 text-sm text-gray-700">
            <p class="font-semibold">{move || state.name().get().unwrap_or_else(|| "User".to_string())}</p>
            <p class="text-xs text-gray-500">{move || state.email().get().unwrap_or_else(|| "No email".to_string())}</p>
        </div>
    }
}

#[component]
pub fn LoginButton(dropdown_open: RwSignal<bool>) -> impl IntoView {
    let on_login = move |_| {
        dropdown_open.set(false);
        // Spawn async task for login
        wasm_bindgen_futures::spawn_local(async move {
            log!("[KeyCloak] Starting login flow...");

            // Configure KeyCloak login parameters
            let params = LoginParams::new(
                crate::auth_config::KEYCLOAK_ISSUER.to_string(),
                crate::auth_config::KEYCLOAK_CLIENT_ID.to_string(),
                crate::auth_config::KEYCLOAK_REDIRECT_URI.to_string(),
            )
            .with_scope(crate::auth_config::KEYCLOAK_SCOPE.to_string())
            .with_challenge(Challenge::S256);

            log!("[KeyCloak] Login params: issuer={}, client_id={}, redirect_uri={}",
                params.issuer, params.client_id, params.redirect_uri);
            
            // This will redirect to KeyCloak, so the code after this won't execute
            match keycloak_wasm_auth::login(params).await {
                Ok(_) => {
                    log!("[KeyCloak] Redirecting to KeyCloak...");
                    // This line won't be reached because login() redirects the browser
                }
                Err(e) => {
                    log!("[KeyCloak] ❌ Login initiation failed: {}", e);
                }
            }
        });
    };

    view! {
        <button
            class="block w-full px-4 py-2 text-left text-sm text-gray-700 hover:bg-gray-100"
            role="menuitem"
            tabindex="-1"
            on:click=on_login
        >
            "Log In with KeyCloak"
        </button>
    }
}

#[component]
pub fn LogoutButton(dropdown_open: RwSignal<bool>) -> impl IntoView {
    let state = expect_context::<Store<GlobalState>>();

    let on_logout = move |_| {
        dropdown_open.set(false);
        log!("[KeyCloak] Logging out...");

        // Clear authentication state
        state.token().set(None);
        state.user_id().set(None);
        state.email().set(None);
        state.username().set(None);
        state.name().set(None);
        state.roles().set(Vec::new());
        state.is_authenticated().set(false);

        match keycloak_wasm_auth::logout() {
            Ok(_) => {
                log!("[KeyCloak] Logout successful.");
                // This line won't be reached because login() redirects the browser
            }
            Err(e) => {
                log!("[KeyCloak] ❌ Logout initiation failed: {}", e);
            }
        }

        log!("[KeyCloak] ✅ Logout successful");
    };

    view! {
        <button
            class="block w-full px-4 py-2 text-left text-sm text-gray-700 hover:bg-gray-100"
            role="menuitem"
            tabindex="-1"
            on:click=on_logout
        >
            "Log Out"
        </button>
    }
}

#[component]
pub fn InitDatabaseButton(dropdown_open: RwSignal<bool>) -> impl IntoView {
    let state = expect_context::<Store<GlobalState>>();
    let token = state.token().get();

    let init_database = store_value(move |_| {
        let token_clone = token.clone();
        leptos::task::spawn_local(async move {
            init_database(token_clone).await;
            dropdown_open.set(false);
        });
        state.refresh_table().patch(Uuid::new_v4().to_string());
    });

    view! {
        <KeycloakAccessAdmin>
        <Button appearance=ButtonAppearance::Primary on_click=move |e| init_database.get_value()(e) class="block w-full">"INIT Database"</Button>
        </KeycloakAccessAdmin>
    }
}