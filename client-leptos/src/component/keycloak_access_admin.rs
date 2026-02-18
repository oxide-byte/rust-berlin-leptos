use leptos::prelude::*;
use reactive_stores::Store;
use crate::component::keycloak_catcher::{GlobalState, GlobalStateStoreFields};

#[component]
pub fn KeycloakAccessAdmin(children: ChildrenFn) -> impl IntoView {
    let state = expect_context::<Store<GlobalState>>();

    let is_admin = Memo::new(move |_| {
        state.roles().get().contains(&"ROLE_HNL_ADMIN".to_string())
    });

    view! {
        <Show when=move || is_admin.get()>
            {children()}
        </Show>
    }
}