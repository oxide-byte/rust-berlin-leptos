use crate::component::NavigationUserMenu;
use leptos::prelude::*;

#[component]
pub fn NavigationBar() -> impl IntoView {
    view! {
    <nav class="bg-gray-800">
      <div class="mx-auto max-w-7xl px-2 sm:px-6 lg:px-8">
        <div class="relative flex h-16 items-center justify-between">
          <div class="flex flex-1 items-center justify-center sm:items-stretch sm:justify-start">
            <div class="flex shrink-0 items-center">
              <h5 class="text-white">Berlin Rust Hack&Learn Web Resources</h5>
            </div>
            <div class="hidden sm:ml-6 sm:block">
            </div>
          </div>
          <div class="absolute inset-y-0 right-0 flex items-center pr-2 sm:static sm:inset-auto sm:ml-6 sm:pr-0">
            <NavigationUserMenu/>
          </div>
        </div>
      </div>
    </nav>
    }
}