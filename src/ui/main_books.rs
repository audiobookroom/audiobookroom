use leptos::{html::Input, *};

use crate::{
    server_api::{auth::User, authors::AuthorDetail, book::BookDetail},
    ui::player::AudioProps,
};
#[derive(Clone, Debug, PartialEq)]
pub enum BookPageContent {
    Index,
    BookDetail((BookDetail, AuthorDetail)),
}
#[component]
pub fn MainBooks(
    current_content: ReadSignal<BookPageContent>,
    set_current_content: WriteSignal<BookPageContent>,
) -> impl IntoView {
    view! {
        {move || {
            match current_content.get() {
                BookPageContent::Index => {
                    view! { <BookIndex set_current_content=set_current_content/> }.into_view()
                }
                BookPageContent::BookDetail((book_detail, author_detail)) => {
                    view! {
                        <BookDetail
                            book_detail=book_detail
                            author_detail=author_detail
                            set_current_content=set_current_content
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
fn BookIndex(#[prop(into)] set_current_content: Callback<BookPageContent, ()>) -> impl IntoView {
    let (current_page, set_current_page) = create_signal(0u64);
    let (max_item, _set_max_item) = create_signal(100);
    let books = create_resource(
        move || (current_page.get(), max_item.get()),
        move |(page_number, max_item)| async move {
            let books = crate::server_api::book::get_books_details(page_number, max_item).await;
            books
        },
    );
    let page_node_ref: NodeRef<Input> = create_node_ref();
    view! {
        <div class="flex flex-col items-center space-y-1 w-full">
            <Transition fallback=move || {
                view! { <p>{"Loading..."}</p> }
            }>
                {move || {
                    books
                        .get()
                        .map(|books| {
                            match books {
                                Ok(books) => {
                                    let number_of_pages = books.number_of_pages;
                                    view! {
                                        <div class="flex flex-row justify-evenly  items-center  space-y-1 space-x-1 flex-wrap w-full">

                                            {if !books.items.is_empty() {
                                                view! { <h1>{"Books:"}</h1> }
                                            } else {
                                                view! { <h1>{"No books"}</h1> }
                                            }}
                                            {books
                                                .items
                                                .into_iter()
                                                .map(|(book, author)| {
                                                    let book_to_send = book.clone();
                                                    let author_to_send = author.clone();
                                                    view! {
                                                        <button
                                                            class="p-1  bg-blue-50 shadow-sm hover:shadow-lg hover:bg-green-50"
                                                            on:click=move |_| {
                                                                set_current_content(
                                                                    BookPageContent::BookDetail((
                                                                        book_to_send.clone(),
                                                                        author_to_send.clone(),
                                                                    )),
                                                                );
                                                            }
                                                        >

                                                            <h1>{&book.name}</h1>
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
fn BookDetail(
    #[prop(into)] book_detail: BookDetail,
    #[prop(into)] author_detail: AuthorDetail,
    #[prop(into)] set_current_content: Callback<BookPageContent, ()>,
) -> impl IntoView {
    use crate::server_api::book::*;

    let set_player_props = use_context::<WriteSignal<Option<AudioProps>>>().unwrap();
    let (current_page, set_current_page) = create_signal(0u64);
    let page_node_ref: NodeRef<Input> = create_node_ref();
    let (max_item, _set_max_item) = create_signal(100);
    let chapters = create_resource(
        move || (current_page.get(), max_item.get()),
        move |(current_page, max_item)| async move {
            let chapters = get_chapters(book_detail.id, current_page, max_item).await;
            chapters
        },
    );
    use crate::server_api::progress::*;
    let user = use_context::<User>().unwrap();
    let _book_detail = book_detail.clone();
    let current_progress = create_resource(
        move || {},
        move |_| {
            let _book_detail = _book_detail.clone();
            async move {
                let current_p = get_progress(book_detail.id, user.id).await.unwrap();

                if let Some(p) = current_p {
                    let chapter_detail = crate::server_api::book::get_chatper_detail(p.chapter_id)
                        .await
                        .unwrap();
                    return Some((p, _book_detail, chapter_detail));
                } else {
                    return None;
                }
            }
        },
    );
    let on_progress_button_click = move |account_id: i32, book_id: i32| {
        spawn_local(async move {
            let progress = crate::server_api::progress::get_progress(book_id, account_id).await;
            if let Ok(Some(k)) = progress {
                set_player_props(Some(crate::ui::player::AudioProps {
                    book_id,
                    chapter_id: k.chapter_id,
                    init_time: k.progress,
                }));
            }
        });
    };
    view! {
        <div class="flex flex-col items-center text-center w-full ">
            <h1>{&book_detail.name}</h1>
            <h2>{&author_detail.name}</h2>
            <Transition fallback=move || {
                view! { <p>{"Loading..."}</p> }
            }>
                <div class="flex-col flex items-center space-y-1 w-full">

                    {move || {
                        current_progress
                            .get()
                            .map(|current_p| {
                                match current_p {
                                    Some((progress_item, book, chapter)) => {
                                        let book_id = progress_item.music_id;
                                        let chapter_id = progress_item.chapter_id;
                                        let init_time = progress_item.progress;
                                        view! {
                                            <button
                                                class="w-full mx-2 px-2 py-1 bg-blue-50 hover:bg-green-50 border border-solid rounded-sm shadow-md hover:shadow-lg"
                                                on:click=move |_e| {
                                                    set_player_props(
                                                        Some(crate::ui::player::AudioProps {
                                                            book_id,
                                                            chapter_id,
                                                            init_time,
                                                        }),
                                                    );
                                                }
                                            >

                                                <h2>{&book.name}</h2>

                                                {move || {
                                                    view! {
                                                        <h3>{&chapter.chapter_name}</h3>

                                                        <p>
                                                            {format!("Current progress: {}", progress_item.progress)}
                                                        </p>
                                                    }
                                                        .into_view()
                                                }}

                                            </button>
                                        }
                                            .into_view()
                                    }
                                    None => view! { <p>{"No Progress"}</p> }.into_view(),
                                }
                            })
                    }}

                </div>
            </Transition>

            <Transition fallback=move || {
                view! { <p>{"Loading..."}</p> }
            }>

                {move || {
                    let chapters = chapters.get();
                    chapters
                        .map(|chapters| {
                            match chapters {
                                Ok(chapters) => {
                                    let number_of_pages = chapters.number_of_pages;
                                    let current_page = chapters.page;
                                    view! {
                                        <div class="flex-col flex space-x-0 w-full my-1 py-1">

                                            {chapters
                                                .items
                                                .into_iter()
                                                .map(|chapter| {
                                                    let chapter_detail = chapter.clone();
                                                    let chapter_name = &chapter_detail.chapter_name;
                                                    view! {
                                                        <button
                                                            class="w-full  px-2 py-1 bg-blue-50 hover:bg-green-50 border border-solid rounded-sm shadow-md hover:shadow-lg"
                                                            on:click=move |_| {

                                                                on_progress_button_click(user.id, book_detail.id);
                                                            }
                                                        >

                                                            {chapter_name}
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
                                                    value=current_page
                                                />
                                                <p>{format!("of [0 to {})", number_of_pages)}</p>
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
                                                        if current_page >= 1 {
                                                            set_current_page(current_page - 1);
                                                        }
                                                    }
                                                >

                                                    {format!("Prev")}
                                                </button>

                                                <button
                                                    class="bg-gray-400 flex-1 shadow-md hover:bg-gray-50 hover:shadow-lg"
                                                    on:click=move |_| {
                                                        if number_of_pages > 0
                                                            && current_page < (number_of_pages - 1)
                                                        {
                                                            set_current_page(current_page + 1);
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

            <button on:click=move |_| set_current_content(BookPageContent::Index)>{"Back"}</button>
        </div>
    }
}
