use crate::component::{EventTableDelete,
                       EventTableEdit,
                       EventTableModal,
                       GlobalState,
};
use crate::graphql::{delete_meetup_url_by_uuid_id,
                     fetch_meetup_url_data,
                     insert_meetup_event,
                     update_meetup_event,
};
use crate::model::{Event, Filter, MeetupUrlEdit};
use leptos::logging::log;
use leptos::prelude::*;
use reactive_stores::Store;
use thaw::*;
use crate::component::keycloak_catcher::GlobalStateStoreFields;

#[component]
pub fn EventTable() -> impl IntoView {
    let state = expect_context::<Store<GlobalState>>();
    let show_modal = RwSignal::new(false);
    let meetup_url_select = RwSignal::new(MeetupUrlEdit::default());
    let page = RwSignal::new(1 as usize);
    let max_size = RwSignal::new("10".to_string());
    let filter_domain = RwSignal::new(String::from(""));
    let filter_title = RwSignal::new(String::from(""));
    let filter_url = RwSignal::new(String::from(""));
    let filter_description = RwSignal::new(String::from(""));

    let (filter, set_filter) = signal(Filter { page: Some(1), size: Some(10), ..Default::default() });
    let fetch_urls = LocalResource::new(move || load_data(filter.get(), state.token().get()));

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
            new_filter.page = Some((page.get() - 1) as i64);
            new_filter.size = Some(max_size.get().parse::<i64>().unwrap());
        }
        set_filter.set(new_filter);
    };

    let add_item = move |_e| {
        meetup_url_select.set(MeetupUrlEdit::default());
        show_modal.set(true);
    };

    let edit_item = move |item: Event| {
        let edit = MeetupUrlEdit {
            uri_uuid: Some(item.id),
            title: Some(item.title),
            domain: Some(item.domain),
            url: Some(item.url),
            description: Some(item.description),
        };
        meetup_url_select.set(edit);
        show_modal.set(true);
    };

    let delete_item = move |item: Event| {
        let token = state.token().get();
        leptos::task::spawn_local(async move {
            delete_meetup_url_by_uuid_id(item.id, token).await;
            fire_refresh();
        });
    };

    fn get_pages(page_size: RwSignal<String>, items: i64) -> impl Into<Signal<usize>> + Sized {
        if page_size.get() == "ALL" {
            0
        } else {
            let ps = page_size.get().parse::<i64>().unwrap();
            items.div_ceil(ps) as usize
        }
    }

    let close_modal = move |item: MeetupUrlEdit| {
        let token = state.token().get();
        if item.uri_uuid.is_none() {
            log!("INSERT {:?}", item);
            leptos::task::spawn_local(async move {
                insert_meetup_event(item, token).await;
                fire_refresh();
            });
        } else {
            log!("UPDATE {:?}", item);
            let token_clone = token.clone();
            leptos::task::spawn_local(async move {
                update_meetup_event(item, token_clone).await;
                fire_refresh();
            });
        }
        show_modal.set(false);
    };
    let cancel_modal = move || {
        show_modal.set(false);
    };

    async fn load_data(filter: Filter, token: Option<String>) -> (Vec<Event>, i64) {
        fetch_meetup_url_data(filter, token).await
    }

    view! {
          <div class="w-full mt-2 mb-2">
              <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                {move || Suspend::new(async move {
                let (urls, count) = fetch_urls.await;

                view! {
                  <p> Count: <strong>{count}</strong> item(s) selected</p>
                  <Table class="w-full table-auto">
                      <TableHeader>
                        <TableRow>
                          <TableHeaderCell>
                            <div class="grid grid-flow-col grid-rows-2">
                                <div>Domain</div>
                                <div class="border ml-1 mr-1"
                                    ><Input value=filter_domain on:change = move |_event| {fire_refresh();}/>
                                </div>
                            </div>
                          </TableHeaderCell>
                          <TableHeaderCell>
                            <div class="grid grid-flow-col grid-rows-2">
                                <div>Title</div>
                                <div class="border ml-1 mr-1">
                                    <Input value=filter_title on:change = move |_event| {fire_refresh();}/>
                                </div>
                            </div>
                          </TableHeaderCell>
                          <TableHeaderCell>
                            <div class="grid grid-flow-col grid-rows-2">
                                <div>URL</div>
                                <div class="border ml-1 mr-1">
                                    <Input value=filter_url on:change = move |_event| {fire_refresh();}/>
                                </div>
                            </div>
                          </TableHeaderCell>
                          <TableHeaderCell>
                            <div class="grid grid-flow-col grid-rows-2">
                                <div>Description</div>
                                <div class="border ml-1 mr-1">
                                    <Input value=filter_description on:change = move |_event| {fire_refresh();}/>
                                </div>
                            </div>
                          </TableHeaderCell>
                          <TableHeaderCell>
                            <div class="relative h-24 w-full">
                                <div class="absolute bottom-4 left-1/2 transform -translate-x-1/2">
                                <Button appearance=ButtonAppearance::Primary on_click=add_item>"ADD ENTRY"</Button>
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
                        {
                            let domain = event.domain.clone();
                            let title = event.title.clone();
                            let url = event.url.clone();
                            let description = event.description.clone();
                            view!{
                                <TableRow>
                                    <TableCell>{{domain}}</TableCell>
                                    <TableCell>{{title}}</TableCell>
                                    <TableCell><Link href=url.clone()>{{url}}</Link></TableCell>
                                    <TableCell>
                                        <Textarea size=TextareaSize::Large resize=TextareaResize::Vertical value=description/>
                                    </TableCell>
                                    <TableCell>
                                            <div class="basis-1/12 flex items-center justify-center">
                                               <div class="flex flex-row-reverse space-x-4 space-x-reverse">
                                                    <EventTableEdit event={event.clone()} on_click=edit_item></EventTableEdit>
                                                    <EventTableDelete event={event.clone()} on_click=delete_item></EventTableDelete>
                                               </div>
                                            </div>
                                    </TableCell>
                                </TableRow>
                            }
                        }
                         </For>
                      </TableBody>
                      <tfoot>
                        <Flex>
                        <Pagination page page_count=get_pages(max_size, count) on:click = move |_event| {fire_refresh();} />
                        <Select value=max_size on:change = move |_event| {fire_refresh();} >
                            <option>10</option>
                            <option>50</option>
                            <option>ALL</option>
                        </Select>
                        </Flex>
                      </tfoot>
                  </Table>
                }})}
              </Suspense>
            <Show when = move || show_modal.get()>
                <EventTableModal meetup_url=meetup_url_select on_close_modal=close_modal on_cancel_modal=cancel_modal/>
            </Show>
          </div>
      }
}