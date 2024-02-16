use leptos::{html::Input, *};

use crate::server_api::authors::AuthorDetail;

use super::{main_books::BookPageContent, main_page::MainPageContent};
#[derive(Clone, Debug, PartialEq)]
pub enum AuthorPageContent {
    Index,
    AuthorDetail(AuthorDetail),
}
#[component]
pub fn MainAuthors(
    #[prop(into)] set_main_content: Callback<MainPageContent, ()>,
    #[prop(into)] set_book_current_content: Callback<BookPageContent, ()>,
) -> impl IntoView {
    let (current_content, set_current_content) = create_signal(AuthorPageContent::Index);
    view! {
        {move || {
            match current_content.get() {
                AuthorPageContent::Index => {
                    view! { <AuthorIndex set_current_content=set_current_content/> }.into_view()
                }
                AuthorPageContent::AuthorDetail(author_detail) => {
                    view! {
                        <AuthorDetailPage
                            author_detail=author_detail
                            set_current_content=set_current_content
                            set_main_content=set_main_content
                            set_book_current_content=set_book_current_content
                        />
                    }
                        .into_view()
                }
            }
        }}
    }
}

#[component]
/// this will show all books
fn AuthorIndex(
    #[prop(into)] set_current_content: Callback<AuthorPageContent, ()>,
) -> impl IntoView {
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
                                                    let author_to_send = author.clone();
                                                    view! {
                                                        <button
                                                            class=" my-1 p-1 bg-blue-50 shadow-sm flex-auto hover:shadow-lg hover:bg-green-50"
                                                            on:click=move |_| {
                                                                set_current_content(
                                                                    AuthorPageContent::AuthorDetail(author_to_send.clone()),
                                                                );
                                                            }
                                                        >

                                                            <h2>{&author.name}</h2>
                                                        </button>
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
fn AuthorDetailPage(
    #[prop(into)] author_detail: AuthorDetail,
    #[prop(into)] set_current_content: Callback<AuthorPageContent, ()>,
    #[prop(into)] set_main_content: Callback<MainPageContent, ()>,
    #[prop(into)] set_book_current_content: Callback<BookPageContent, ()>,
) -> impl IntoView {
    use crate::server_api::book::*;

    let (current_page, set_current_page) = create_signal(0u64);
    let page_node_ref: NodeRef<Input> = create_node_ref();
    let (max_item, _set_max_item) = create_signal(20);
    let author_id = author_detail.id;
    let books = create_resource(
        move || (current_page.get(), max_item.get()),
        move |(current_page, max_item)| async move {
            let books = get_books_by_author(author_id, current_page, max_item).await;
            books
        },
    );

    view! {
        <div class="flex flex-col items-center space-y-1 w-full">
            <Transition fallback=move || {
                view! { <p>{"Loading..."}</p> }
            }>

                {
                    let author_detail = author_detail.clone();
                    move || {
                        let author_detail = author_detail.clone();
                        books
                            .get()
                            .map(|books| {
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
                                                        let book_to_send = book.clone();
                                                        let author_name = &author_detail.name;
                                                        let author_detail = author_detail.clone();
                                                        view! {
                                                            <button
                                                                class=" w-full bg-blue-50 shadow-sm hover:shadow-lg hover:bg-green-50"
                                                                on:click=move |_| {
                                                                    set_book_current_content(
                                                                        BookPageContent::BookDetail((
                                                                            book_to_send.clone(),
                                                                            author_detail.clone(),
                                                                        )),
                                                                    );
                                                                    set_main_content(MainPageContent::Books);
                                                                }
                                                            >

                                                                <h1>{&book.name}</h1>
                                                                <h2>{author_name}</h2>
                                                            </button>
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
                                                    <button
                                                        class="bg-gray-400  shadow-md hover:bg-gray-50 hover:shadow-lg"
                                                        on:click=move |_| {
                                                            set_current_content(AuthorPageContent::Index);
                                                        }
                                                    >

                                                        <h1>{"Back"}</h1>
                                                    </button>
                                                </div>
                                            </div>
                                        }
                                            .into_view()
                                    }
                                    Err(e) => {
                                        view! { <p>{format!("Error: {:?}", e)}</p> }.into_view()
                                    }
                                }
                            })
                    }
                }

            </Transition>

        </div>
    }
}
