use leptos::prelude::*;
use crate::models::event::generate_demo;

#[component]
pub fn EventTable() -> impl IntoView {
    let data = RwSignal::new(generate_demo(10));
    view! {
          <div class="w-full">
              <table class="w-full table-auto">
                  <thead>
                    <tr>
                      <th>
                        <div class="grid grid-flow-col grid-rows-2">
                            <div>Domain</div>
                            <div class="border ml-1 mr-1"><input/></div>
                        </div>
                      </th>
                      <th>
                        <div class="grid grid-flow-col grid-rows-2">
                            <div>Title</div>
                            <div class="border ml-1 mr-1"><input/></div>
                        </div>
                      </th>
                      <th>
                        <div class="grid grid-flow-col grid-rows-2">
                            <div>URL</div>
                            <div class="border ml-1 mr-1"><input/></div>
                        </div>
                      </th>
                      <th>
                        <div class="grid grid-flow-col grid-rows-2">
                            <div>Description</div>
                            <div class="border ml-1 mr-1"><input/></div>
                        </div>
                      </th>
                    </tr>
                  </thead>
                  <tbody>
                    <For
                        each=move || data.get()
                        key=|item| item.id.clone()
                        let:event
                    >
                        <tr>
                            <td>{{event.title}}</td>
                            <td>{{event.domain}}</td>
                            <td><a href={{event.url.clone()}} class="font-medium text-blue-600 dark:text-blue-500 hover:underline"> {{event.url.clone()}} </a></td>
                            <td>{{event.description}}</td>
                        </tr>
                     </For>
                  </tbody>
                  <tfoot>
                      <tr>
                        <td>Showing....</td>
                        <td>Pages....</td>
                      </tr>
                  </tfoot>
              </table>
          </div>
      }
}