use leptos::html::{div, h2, img, strong};
use leptos::prelude::*;

#[component]
pub fn Banner() -> impl IntoView {

    // BUILDER STYLE

    let image = img()
        .alt("Logo")
        .src("./public/browsing_ferris.png")
        .class("h-30 w-30");

    let title = div()
        .class("grow content-center text-center").child(
        div().class("text-3xl")
            .child(strong().child("Summer of Rust Web Frameworks"))
    );

    let version = h2()
        .class("text-3xl")
        .child("Leptos Version");

    let build = div().class("flex flex-row")
        .child(
            div()
                .class("grow content-center text-center")
                .child((image, title, version))
        );

    build.into_view()
}
/*
    // HTML STYLE

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
    */