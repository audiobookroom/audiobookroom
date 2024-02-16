use leptos::{html::Input, *};

use crate::server_api::book::AddBookResult;

#[derive(Clone, Debug, PartialEq)]
pub enum SettingsContent {
    Main,
    AddBook,
    DeleteBook,
    AddUser,
    DeleteUser,
}

#[component]
pub fn MainSettings() -> impl IntoView {
    let (current_content, set_current_content) = create_signal(SettingsContent::Main);

    view! {
        {move || {
            match current_content.get() {
                SettingsContent::Main => {
                    view! {
                        <div class="flex flex-col items-center w-full">
                            <h1>{"Settings"}</h1>
                            <button
                                class="w-full text-center bg-blue-50 hover:bg-green-50 px-1 py-1 my-2 border border-solid rounded-full"
                                on:click=move |_| { set_current_content(SettingsContent::AddBook) }
                            >
                                {"Add Book"}
                            </button>
                            <button
                                class="w-full text-center bg-blue-50 hover:bg-green-50 px-1 py-1 my-2 border border-solid rounded-full"
                                on:click=move |_| {
                                    set_current_content(SettingsContent::DeleteBook)
                                }
                            >

                                {"Delete Book"}
                            </button>
                            <button
                                class="w-full text-center bg-blue-50 hover:bg-green-50 px-1 py-1 my-2 border border-solid rounded-full"
                                on:click=move |_| { set_current_content(SettingsContent::AddUser) }
                            >
                                {"Add User"}
                            </button>
                            <button
                                class="w-full text-center bg-blue-50 hover:bg-green-50 px-1 py-1 my-2 border border-solid rounded-full"
                                on:click=move |_| {
                                    set_current_content(SettingsContent::DeleteUser)
                                }
                            >

                                {"Delete User"}
                            </button>
                        </div>
                    }
                        .into_view()
                }
                SettingsContent::AddBook => view! { <AddBook/> }.into_view(),
                SettingsContent::DeleteBook => view! { <DeleteBook/> }.into_view(),
                SettingsContent::AddUser => view! { <AddUser/> }.into_view(),
                SettingsContent::DeleteUser => view! { <DeleteUser/> }.into_view(),
            }
        }}
    }
}

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

#[component]
pub fn DeleteBook() -> impl IntoView {
    let (page, set_page) = create_signal(0);
    let (max_page, _set_max_page) = create_signal(100);
    let delete_book = create_server_action::<crate::server_api::book::DeleteBook>();
    let all_books = create_resource(
        move || (page.get(), max_page.get(), delete_book.version().get()),
        |(page, max_page, _)| async move {
            let books = crate::server_api::book::get_books(page, max_page).await;
            books
        },
    );
    let page_node_ref = create_node_ref::<Input>();
    view! {
        <h2>{"Delete Book"}</h2>
        <Transition fallback=move || {
            view! { <span>"Loading..."</span> }
        }>
            {move || {
                all_books
                    .get()
                    .map(|books| {
                        let books = books.unwrap();
                        let number_of_pages = books.number_of_pages;
                        view! {
                            <div class="flex flex-col items-center w-full">
                                <h1>{"Books"}</h1>
                                <div class="flex flex-col items-center w-full">

                                    {books
                                        .items
                                        .into_iter()
                                        .map(|book| {
                                            view! {
                                                <button
                                                    class="w-full text-center bg-blue-50 hover:bg-green-50 px-1 py-1 my-2 border border-solid rounded-full"
                                                    on:click=move |_e| {
                                                        delete_book.dispatch(book.id.into());
                                                    }
                                                >

                                                    {&book.name}

                                                </button>
                                            }
                                        })
                                        .collect_view()}

                                </div>
                                // the pager

                                <div class="flex flex-col items-center w-full">
                                    <div class="flex flex-col w-full space-y-1">
                                        <div class="flex flex-row items-center justify-between w-full space-x-2">
                                            <input
                                                ref=page_node_ref
                                                class="flex-1 px-1"
                                                value=books.page
                                            />
                                            <p>{format!("of [0 to {})", books.number_of_pages)}</p>
                                            <button
                                                on:click=move |_| {
                                                    let page_num = page_node_ref
                                                        .get()
                                                        .unwrap()
                                                        .value()
                                                        .parse::<u64>()
                                                        .unwrap();
                                                    if page_num < number_of_pages {
                                                        set_page(page_num);
                                                    }
                                                }

                                                class="flex-1 px-2 bg-gray-400  shadow-md hover:bg-gray-50 hover:shadow-lg"
                                            >
                                                go
                                            </button>
                                        </div>
                                        <div class="flex flex-row items-center justify-between w-full space-x-2">

                                            <button
                                                class="bg-gray-400 flex-1 shadow-md hover:bg-gray-50 hover:shadow-lg"
                                                on:click=move |_| {
                                                    if books.page >= 1 {
                                                        set_page(books.page - 1);
                                                    }
                                                }
                                            >

                                                {format!("Prev")}
                                            </button>

                                            <button
                                                class="bg-gray-400 flex-1 shadow-md hover:bg-gray-50 hover:shadow-lg"
                                                on:click=move |_| {
                                                    if books.number_of_pages > 0
                                                        && books.page < (books.number_of_pages - 1)
                                                    {
                                                        set_page(books.page + 1);
                                                    }
                                                }
                                            >

                                                {format!("Next")}
                                            </button>
                                        </div>
                                    </div>
                                // end pager
                                </div>
                            </div>
                        }
                    })
            }}

        </Transition>
    }
}

#[component]
pub fn AddUser() -> impl IntoView {
    view! {
        <h2>{"Add User"}</h2>
        <h2>{"TODO unimplemented yet"}</h2>
    }
}

#[component]
pub fn DeleteUser() -> impl IntoView {
    view! {
        <h2>{"Delete User"}</h2>
        <h2>{"TODO unimplemented yet"}</h2>
    }
}
