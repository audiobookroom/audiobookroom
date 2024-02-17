use leptos::{html::Input, *};

use crate::server_api::book::AddBookResult;
#[component]
pub fn AddBook() -> impl IntoView {
    let (add_info, set_add_info) = create_signal(None);
    let name_node = create_node_ref::<Input>();
    let author_node = create_node_ref::<Input>();
    let source_node = create_node_ref::<Input>();

    let add_result = create_resource(
        move || add_info.get(),
        move |add_info| async move {
            // add_info.map(|(name, author, source)| async {
            //     let result = crate::server_api::book::add_book(author, name, source).await;
            //     result
            // })
            if let Some((name, author, source)) = add_info {
                let result = crate::server_api::book::add_book(author, name, source).await;
                Some(result)
            } else {
                None
            }
        },
    );
    view! {
        <div class="flex-col flex items-start space-y-2 w-full px-1 py-1 ">
            <h1>{"Add Book"}</h1>
            <h2>Book Name:</h2>
            <input
                class="w-full  my-1 px-4 py-1 bg-gray-100 hover:bg-gray-50 hover:shadow-lg border border-solid rounded-full"
                type="text"
                ref=name_node
                placeholder="Book Name"
            />
            <h2>Author Name:</h2>
            <input
                class="w-full  my-1 px-4 py-1 bg-gray-100 hover:bg-gray-50 hover:shadow-lg border border-solid rounded-full"
                type="text"
                ref=author_node
                placeholder="Author"
            />
            <h2>File Directory:</h2>
            <input
                class="w-full  my-1 px-4 py-1 bg-gray-100 hover:bg-gray-50 hover:shadow-lg border border-solid rounded-full"
                type="text"
                ref=source_node
                placeholder="Source Directory"
            />
            <div class="w-full py-12">
                <button
                    class="w-full   px-1 py-1  bg-gray-400 hover:bg-gray-50 hover:shadow-lg border border-solid rounded-full"
                    on:click=move |_| {
                        set_add_info(
                            Some((
                                name_node.get().unwrap().value(),
                                author_node.get().unwrap().value(),
                                source_node.get().unwrap().value(),
                            )),
                        )
                    }
                >

                    {"Submit"}
                </button>
            </div>

        </div>
        <Transition fallback=move || {
            view! { <span>"Loading..."</span> }
        }>

            {move || {
                add_result
                    .get()
                    .map(|add_result: Option<Result<AddBookResult, ServerFnError>>| {
                        match add_result {
                            Some(Ok(_)) => view! { <span>"Add Success"</span> },
                            Some(Err(e)) => view! { <span>{format!("Add Failed: {}", e)}</span> },
                            None => view! { <span>"Click Submit to add the book"</span> },
                        }
                    })
            }}

        </Transition>
    }
}
