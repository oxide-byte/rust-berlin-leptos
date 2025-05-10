use leptos::prelude::*;
use crate::model::event::Event;

#[component]
pub fn EventTableDelete<F>(#[prop(into)] event: Event, on_click: F) -> impl IntoView
where
    F: Fn(Event) + 'static + Copy,
{
    let button_del_class = "text-white bg-red-700 hover:bg-red-800 focus:ring-4 focus:outline-none focus:ring-red-300 font-medium rounded-full text-sm p-2.5 text-center inline-flex items-center mr-2";
    let event: RwSignal<Event> = RwSignal::new(event);

    let delete = move |_| {
        on_click(event.get());
    };

    view! {
        <button
            class=button_del_class
            on:click=delete>
            <i class="fa-solid fa-minus"></i>
        </button>
    }
}