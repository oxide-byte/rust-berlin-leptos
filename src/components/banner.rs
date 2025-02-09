use leptos::prelude::*;

#[component]
pub fn Banner() -> impl IntoView {
    view! {
        <div class="flex flex-row">
            <div>
                <img src="./public/browsing_ferris.png" alt="Logo" class="h-30 w-30"/>
            </div>
            <div class="grow content-center text-center">
                <div class="mb-2">
                    <h2 class="text-3xl"><b>Summer of Rust Web Frameworks</b></h2>
                </div>
                <div>
                    <h2 class="text-3xl">Leptos Version</h2>
                </div>
            </div>
        </div>
    }
}