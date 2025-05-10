use crate::model::meetup_url_edit::MeetupUrlEdit;
use leptos::html::Input;
use leptos::prelude::*;

#[component]
pub fn EventTableModal<F, R>(#[prop(
    into
)] meetup_url: RwSignal<MeetupUrlEdit>, on_close_modal: F, on_cancel_modal: R) -> impl IntoView
where
    F: Fn(MeetupUrlEdit) + 'static + Copy,
    R: Fn() + 'static + Copy,
{
    let input_field_class = "shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline";

    let is_update = meetup_url.get().uri_uuid.is_some();

    let (title, _set_title) = signal(meetup_url.get().title);
    let title_node: NodeRef<Input> = NodeRef::new();

    let (domain, _set_domain) = signal(meetup_url.get().domain);
    let domain_node: NodeRef<Input> = NodeRef::new();

    let (description, _set_description) = signal(meetup_url.get().description);
    let description_node: NodeRef<Input> = NodeRef::new();

    let (url, _set_url) = signal(meetup_url.get().url);
    let url_node: NodeRef<Input> = NodeRef::new();

    let submit = move |_| {
        let title = title_node
            .get()
            .expect("<title> should be mounted")
            .value();

        let description = description_node
            .get()
            .expect("<description> should be mounted")
            .value();

        let url = url_node
            .get()
            .expect("<url> should be mounted")
            .value();

        let domain = domain_node
            .get()
            .expect("<domain> should be mounted")
            .value();

        let mut rtn = meetup_url.get();

        rtn.title = Some(title);
        rtn.description = Some(description);
        rtn.url = Some(url);
        rtn.domain = Some(domain);

        if is_update {
            rtn.uri_uuid = meetup_url.get().uri_uuid;
        };

        on_close_modal(rtn);
    };

    let cancel = move |_| {
        on_cancel_modal();
    };

    view! {
        <div class="fixed inset-0 z-50 flex items-center justify-center bg-gray-600 bg-opacity-90">
            <div class="block rounded-lg bg-white w-2/5 p-4 shadow-[0_2px_15px_-3px_rgba(0,0,0,0.07),0_10px_20px_-2px_rgba(0,0,0,0.04)] z-70">
                
             <h5 class="mb-5 text-xl font-medium leading-tight text-neutral-800">
                Meetup URL
            </h5>

                <div class="mb-5">
                    <label class="block text-gray-700 text-sm font-bold mb-2 mt-2" for="title">
                        Title
                    </label>
                    <input
                        node_ref=title_node
                        class=input_field_class
                        id="title"
                        type="text"
                        value=title
                        placeholder="Title"/>
                    
                    <label class="block text-gray-700 text-sm font-bold mb-2 mt-2" for="description">
                        Description
                    </label>
                    <input
                        node_ref=description_node
                        class=input_field_class
                        id="description"
                        type="text"
                        value=description
                        placeholder="Description"/>
                        
                    <label class="block text-gray-700 text-sm font-bold mb-2 mt-2" for="url">
                        URL
                    </label>
                    <input
                        node_ref=url_node
                        class=input_field_class
                        id="url"
                        type="text"
                        value=url
                        placeholder="Url"/>
        
                    <label class="block text-gray-700 text-sm font-bold mb-2 mt-2" for="domain">
                        Domain
                    </label>
                    <input
                        node_ref=domain_node
                        class=input_field_class
                        id="domain"
                        type="text"
                        value=domain
                        placeholder="Domain"/>        
                </div>               

                <div class="flex flex-row-reverse space-x-4 space-x-reverse">
                    <button type="submit"
                        on:click=submit
                        class="bg-blue-700 hover:bg-blue-800 px-5 py-3 text-white rounded-lg">
                        Save
                    </button>
                    <button type="cancel"
                        on:click=cancel
                        class="bg-gray-300 hover:bg-gray-400 px-5 py-3 text-white rounded-lg">
                        Cancel
                    </button>
                </div>
            </div>
        </div>
    }
}