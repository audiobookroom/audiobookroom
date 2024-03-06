use leptos::{html::Input, *};
use leptos_router::{Route, A};

use crate::server_api::authors::get_author_by_id;

#[component(transparent)]
pub fn MainAuthors() -> impl IntoView {
    view! {
        <Route path="" view=AuthorIndex/>
        <Route path="auhtor/:author_id" view=AuthorDetailPage/>
    }
}

#[component]
/// this will show all books
pub fn AuthorIndex() -> impl IntoView {
    let (current_page, set_current_page) = create_signal(0u64);
    let (max_item, _set_max_item) = create_signal(100);
    let authors = create_resource(
        move || (current_page.get(), max_item.get()),
        move |(page_number, max_item)| async move {
            let authors = crate::server_api::authors::list_all_authors(page_number, max_item).await;
            authors
        },
    );
    let page_node_ref: NodeRef<Input> = create_node_ref();
    view! {
        <div class="flex flex-col items-center space-y-1 w-full">
            <Transition fallback=move || {
                view! { <p>{"Loading..."}</p> }
            }>
                {move || {
                    authors
                        .get()
                        .map(|authors| {
                            match authors {
                                Ok(authors) => {
                                    let number_of_pages = authors.number_of_pages;
                                    view! {
                                        {if !authors.items.is_empty() {
                                            view! { <h1>{"authors:"}</h1> }
                                        } else {
                                            view! { <h1>{"No authors"}</h1> }
                                        }}

                                        <div class="flex flex-row justify-between  items-center  space-x-1 w-full flex-wrap">

                                            {authors
                                                .items
                                                .into_iter()
                                                .map(|author| {
                                                    view! {
                                                        <A
                                                            class=" my-1 p-1 bg-blue-50 shadow-sm flex-auto hover:shadow-lg hover:bg-green-50"
                                                            href=move || format!("auhtor/{}", author.id)
                                                        >

                                                            <h2>{&author.name}</h2>
                                                        </A>
                                                    }
                                                })
                                                .collect::<Vec<_>>()}
                                        </div>

                                        <div class="flex flex-col w-full space-y-1">
                                            <div class="flex flex-row items-center justify-between w-full space-x-2">
                                                <input
                                                    ref=page_node_ref
                                                    class="flex-1 px-1"
                                                    value=authors.page
                                                />
                                                <p>{format!("of [0 to {})", authors.number_of_pages)}</p>
                                                <button
                                                    on:click=move |_| {
                                                        let page_num = page_node_ref
                                                            .get()
                                                            .unwrap()
                                                            .value()
                                                            .parse::<u64>()
                                                            .unwrap();
                                                        if page_num < number_of_pages {
                                                            set_current_page(page_num);
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
                                                        if authors.page >= 1 {
                                                            set_current_page(authors.page - 1);
                                                        }
                                                    }
                                                >

                                                    {format!("Prev")}
                                                </button>

                                                <button
                                                    class="bg-gray-400 flex-1 shadow-md hover:bg-gray-50 hover:shadow-lg"
                                                    on:click=move |_| {
                                                        if authors.number_of_pages > 0
                                                            && authors.page < (authors.number_of_pages - 1)
                                                        {
                                                            set_current_page(authors.page + 1);
                                                        }
                                                    }
                                                >

                                                    {format!("Next")}
                                                </button>
                                            </div>
                                        </div>
                                    }
                                        .into_view()
                                }
                                Err(e) => view! { <p>{format!("Error: {:?}", e)}</p> }.into_view(),
                            }
                        })
                }}

            </Transition>

        </div>
    }
}
#[component]
pub fn AuthorDetailPage() -> impl IntoView {
    use crate::server_api::book::*;
    use leptos::*;
    use leptos_router::*;

    #[derive(Params, Clone, PartialEq)]
    struct Para {
        author_id: i32,
    }
    let para = use_params::<Para>();

    let (current_page, set_current_page) = create_signal(0u64);
    let page_node_ref: NodeRef<Input> = create_node_ref();
    let (max_item, _set_max_item) = create_signal(20);
    let books_author = create_resource(
        move || {
            (
                para.get().unwrap().author_id,
                current_page.get(),
                max_item.get(),
            )
        },
        move |(author_id, current_page, max_item)| async move {
            let author_detail = get_author_by_id(author_id).await.unwrap().unwrap();
            let books = get_books_by_author(author_id, current_page, max_item).await;
            (books, author_detail)
        },
    );

    view! {
        <div class="flex flex-col items-center space-y-1 w-full">
            <Transition fallback=move || {
                view! { <p>{"Loading..."}</p> }
            }>

                {move || {
                    books_author
                        .get()
                        .map(|(books, author_detail)| {
                            let author_name = author_detail.name.clone();
                            match books {
                                Ok(books) => {
                                    let number_of_pages = books.number_of_pages;
                                    view! {
                                        <div class="flex flex-col  items-center  space-y-1 w-full">

                                            {if !books.items.is_empty() {
                                                view! { <h1>{"Books:"}</h1> }
                                            } else {
                                                view! { <h1>{"No books"}</h1> }
                                            }}
                                            {books
                                                .items
                                                .into_iter()
                                                .map(|book| {
                                                    let author_name = author_name.clone();
                                                    view! {
                                                        <A
                                                            class=" w-full bg-blue-50 shadow-sm hover:shadow-lg hover:bg-green-50"
                                                            href=move || format!("/books/book/{}", book.id)
                                                        >
                                                            <h1>{&book.name}</h1>
                                                            <h2>{author_name}</h2>
                                                        </A>
                                                    }
                                                })
                                                .collect::<Vec<_>>()}
                                        </div>

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
                                                            set_current_page(page_num);
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
                                                            set_current_page(books.page - 1);
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
                                                            set_current_page(books.page + 1);
                                                        }
                                                    }
                                                >

                                                    {format!("Next")}
                                                </button>
                                            </div>
                                            <div class="flex flex-row w-full my-1 justify-center">
                                                <A
                                                    class="bg-gray-400  shadow-md hover:bg-gray-50 hover:shadow-lg"
                                                    href="/authors"
                                                >

                                                    <h1>{"Back"}</h1>
                                                </A>
                                            </div>
                                        </div>
                                    }
                                        .into_view()
                                }
                                Err(e) => view! { <p>{format!("Error: {:?}", e)}</p> }.into_view(),
                            }
                        })
                }}

            </Transition>

        </div>
    }
}
