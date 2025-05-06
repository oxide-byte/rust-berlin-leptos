use crate::component::event_table_delete::EventTableDelete;
use crate::component::event_table_edit::EventTableEdit;
use crate::graphql::meetup_url_graphql::fetch_meetup_url_data;
use crate::model::event::Event;
use crate::model::filter::Filter;
use leptos::prelude::*;
use std::ops::Div;
use thaw::*;

#[component]
pub fn EventTable() -> impl IntoView {
    let page = RwSignal::new(0 as usize);
    let page_count = RwSignal::new(3 as usize);
    let max_size = RwSignal::new("10".to_string());
    let filter_domain = RwSignal::new(String::from(""));
    let filter_title = RwSignal::new(String::from(""));
    let filter_url = RwSignal::new(String::from(""));
    let filter_description = RwSignal::new(String::from(""));

    let (filter, set_filter) = signal(Filter { page: Some(1), size: Some(10), ..Default::default() });
    let fetch_urls = LocalResource::new(move || load_data(filter.get()));

    let fire_refresh = move || {
        let mut new_filter = Filter::default();
        if filter_domain.get() != "" {
            new_filter.domain = Some(filter_domain.get());
        };
        if filter_title.get() != "" {
            new_filter.title = Some(filter_title.get());
        };
        if filter_url.get() != "" {
            new_filter.url = Some(filter_url.get());
        };
        if filter_description.get() != "" {
            new_filter.description = Some(filter_description.get());
        };
        if max_size.get() != "ALL" {
            new_filter.page = Some(page.get() as i64);
            new_filter.size = Some(max_size.get().parse::<i64>().unwrap());
        }
        set_filter.set(new_filter);
    };

    let edit_item = move |item| {};

    let delete_item = move |item| {
        fire_refresh();
    };

    async fn load_data(filter: Filter) -> (Vec<Event>, i64) {
        fetch_meetup_url_data(filter).await
    }

    view! {
          <div class="w-full mt-2 mb-2">
              <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                {move || Suspend::new(async move {
                let (urls, count) = fetch_urls.await;

                view! {
                  <p> Count: <strong>{count}</strong> item(s) in Database</p>
                  <Table class="w-full table-auto">
                      <TableHeader>
                        <TableRow>
                          <TableHeaderCell>
                            <div class="grid grid-flow-col grid-rows-2">
                                <div>Domain</div>
                                <div class="border ml-1 mr-1"
                                    ><Input value=filter_domain on:change = move |event| {fire_refresh();}/>
                                </div>
                            </div>
                          </TableHeaderCell>
                          <TableHeaderCell>
                            <div class="grid grid-flow-col grid-rows-2">
                                <div>Title</div>
                                <div class="border ml-1 mr-1">
                                    <Input value=filter_title on:change = move |event| {fire_refresh();}/>
                                </div>
                            </div>
                          </TableHeaderCell>
                          <TableHeaderCell>
                            <div class="grid grid-flow-col grid-rows-2">
                                <div>URL</div>
                                <div class="border ml-1 mr-1">
                                    <Input value=filter_url on:change = move |event| {fire_refresh();}/>
                                </div>
                            </div>
                          </TableHeaderCell>
                          <TableHeaderCell>
                            <div class="grid grid-flow-col grid-rows-2">
                                <div>Description</div>
                                <div class="border ml-1 mr-1">
                                    <Input value=filter_description on:change = move |event| {fire_refresh();}/>
                                </div>
                            </div>
                          </TableHeaderCell>
                          <TableHeaderCell>
                            <div class="relative h-24 w-full">
                                <div class="absolute bottom-4 left-1/2 transform -translate-x-1/2">
                                <Button appearance=ButtonAppearance::Primary>"ADD ENTRY"</Button>
                                </div>
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
                                <TableCell>
                                        <div class="basis-1/12 flex items-center justify-center">
                                           <div class="flex flex-row-reverse space-x-4 space-x-reverse">
                                                <EventTableEdit url_id={event.id.clone()} on_click=edit_item></EventTableEdit>
                                                <EventTableDelete url_id={event.id.clone()} on_click=delete_item></EventTableDelete>
                                           </div>
                                        </div>
                                </TableCell>
                            </TableRow>
                         </For>
                      </TableBody>
                      <tfoot>
                        <Flex>
                        <Pagination page page_count on:click = move |event| {fire_refresh();} />
                        <Select value=max_size default_value="10" on:change = move |_event| {fire_refresh();} >
                            <option>"10"</option>
                            <option>"50"</option>
                            <option>"ALL"</option>
                        </Select>
                        </Flex>
                      </tfoot>
                  </Table>
                }})}
              </Suspense>
          </div>
      }
}