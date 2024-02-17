use leptos::{html::Input, *};

use super::SettingsContent;
#[component]
pub fn DeleteBook(set_content: WriteSignal<SettingsContent>) -> impl IntoView {
    let (page, set_page) = create_signal(0);
    let (max_page, _set_max_page) = create_signal(100);
    let all_books = create_resource(
        move || (page.get(), max_page.get()),
        |(page, max_page)| async move {
            let books = crate::server_api::book::get_books_details(page, max_page).await;
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
                                        .map(|(book, author)| {
                                            view! {
                                                <button
                                                    class="w-full text-center bg-blue-50 hover:bg-green-50 px-1 py-1 my-2 border border-solid rounded-full"
                                                    on:click=move |_e| {
                                                        set_content(SettingsContent::DeleteBookDetail(book.id));
                                                    }
                                                >

                                                    <h1>{&book.name}</h1>
                                                    <h2>{&author.name}</h2>
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
pub fn DeleteBookDetail(book_id: i32, set_content: WriteSignal<SettingsContent>) -> impl IntoView {
    let delete_book = create_server_action::<crate::server_api::book::DeleteBook>();

    let book_detail = create_resource(
        move || delete_book.version().get(),
        move |_| async move {
            let book = crate::server_api::book::get_book_detail(book_id).await;
            if let Ok(book) = book {
                let author = crate::server_api::authors::get_author_by_id(book.author_id).await;
                if let Ok(Some(author)) = author {
                    return Some((book, author));
                }
            }
            None
        },
    );
    let pending = delete_book.pending();
    let done = delete_book.value();
    view! {
        <Transition fallback=move || {
            view! { <span>"Loading..."</span> }
        }>
            {move || {
                book_detail
                    .get()
                    .map(|book_author| {
                        if let Some((book, author)) = book_author {
                            view! {
                                <div class="flex flex-col items-center w-full">
                                    <h1>{"Book Detail"}</h1>
                                    <h2>{format!("Book Name: {}", book.name)}</h2>
                                    <h2>{format!("Author Name: {}", author.name)}</h2>
                                    <button
                                        class="w-full text-center bg-blue-50 hover:bg-green-50 px-1 py-1 my-2 border border-solid rounded-full"
                                        on:click=move |_| {
                                            delete_book.dispatch(book.id.into());
                                        }
                                    >

                                        {"Delete"}
                                    </button>
                                </div>
                            }
                                .into_view()
                        } else {
                            view! { <span>"(deleted)Book not found"</span> }.into_view()
                        }
                    })
            }}

        </Transition>
        <div class="w-full">
            <h1 class="w-full text-center">
                {move || {
                    if pending.get() {
                        "Deleting... please wait"
                    } else if done.get().is_some() {
                        "Done"
                    } else {
                        ""
                    }
                }}

            </h1>
            <button

                class="w-full text-center bg-blue-50 hover:bg-green-50 px-1 py-1 my-2 border border-solid rounded-full"
                on:click=move |_| { set_content(SettingsContent::DeleteBook) }
            >
                {"Back"}
            </button>
        </div>
    }
}
