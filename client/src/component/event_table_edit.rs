use leptos::prelude::*;

#[component]
pub fn EventTableEdit<F>(#[prop(into)] url_id: String, on_click: F) -> impl IntoView
where
    F: Fn(String) + 'static + Copy,
{
    let button_mod_class = "text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-full text-sm p-2.5 text-center inline-flex items-center mr-2";

    view! {
        <button
            class=button_mod_class
            on:click=|e| {println!("EDIT")}>
            <i class="fa-solid fa-edit"></i>
        </button>
    }
}