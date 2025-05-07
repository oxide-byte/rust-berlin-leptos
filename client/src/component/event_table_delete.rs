use leptos::prelude::*;

#[component]
pub fn EventTableDelete<F>(#[prop(into)] url_id: String, on_click: F) -> impl IntoView
where
    F: Fn(String) + 'static + Copy,
{
    let button_del_class = "text-white bg-red-700 hover:bg-red-800 focus:ring-4 focus:outline-none focus:ring-red-300 font-medium rounded-full text-sm p-2.5 text-center inline-flex items-center mr-2";

    let id: RwSignal<String> = RwSignal::new(url_id);

    let delete = move |_| {
        on_click(id.get());
    };

    view! {
        <button
            class=button_del_class
            on:click=delete>
            <i class="fa-solid fa-minus"></i>
        </button>
    }
}