use chrono::{DateTime, Utc};
use leptos::logging::log;
use leptos::prelude::*;
use thaw::*;
use crate::models::event::Event;
use crate::graphql::meetup_url_graph_ql::fetch_meetup_url_data;

#[component]
pub fn EventTable() -> impl IntoView {

    let page = RwSignal::new(1);
    let filter_domain = RwSignal::new(String::from(""));
    let filter_title = RwSignal::new(String::from(""));
    let filter_url = RwSignal::new(String::from(""));
    let filter_description = RwSignal::new(String::from(""));

    let refresh :RwSignal<DateTime<Utc>> = RwSignal::new(Utc::now());
    let fetch_urls = LocalResource::new(move || load_data(refresh.get()));

    async fn load_data(_trigger: DateTime<Utc>) -> Vec<Event> {
        fetch_meetup_url_data().await
    }

    let _effect_page = Effect::watch(
        move || page.get(),
              move |new_page, _, _| {
                  // Query new Page
                  log!("Load new page [{}]", new_page);
                  }, false, );

    view! {
          <div class="w-full mt-2 mb-2">
              <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                {move || Suspend::new(async move {
                let urls = fetch_urls.await;
                view! {
                  <Table class="w-full table-auto">
                      <TableHeader>
                        <TableRow>
                          <TableHeaderCell>
                            <div class="grid grid-flow-col grid-rows-2">
                                <div>Domain</div>
                                <div class="border ml-1 mr-1"><Input value=filter_domain/></div>
                            </div>
                          </TableHeaderCell>
                          <TableHeaderCell>
                            <div class="grid grid-flow-col grid-rows-2">
                                <div>Title</div>
                                <div class="border ml-1 mr-1"><Input value=filter_title/></div>
                            </div>
                          </TableHeaderCell>
                          <TableHeaderCell>
                            <div class="grid grid-flow-col grid-rows-2">
                                <div>URL</div>
                                <div class="border ml-1 mr-1"><Input value=filter_url/></div>
                            </div>
                          </TableHeaderCell>
                          <TableHeaderCell>
                            <div class="grid grid-flow-col grid-rows-2">
                                <div>Description</div>
                                <div class="border ml-1 mr-1"><Input value=filter_description/></div>
                            </div>
                          </TableHeaderCell>
                        </TableRow>
                      </TableHeader>
                      <TableBody>
                        <For
                            each=move || urls.clone()
                            key=|item| item.id.clone()
                            let:event
                        >
                            <TableRow>
                                <TableCell>{{event.domain}}</TableCell>
                                <TableCell>{{event.title}}</TableCell>
                                <TableCell><Link href=event.url.clone()>{{event.url.clone()}}</Link></TableCell>
                                <TableCell>{{event.description}}</TableCell>
                            </TableRow>
                         </For>
                      </TableBody>
                      <tfoot>
                        <Space vertical=true>
                            <div>"Page: " {move || page.get()}</div>
                            <Pagination page page_count=10 />
                        </Space>
                      </tfoot>
                  </Table>
                }})}
              </Suspense>
          </div>
      }
}