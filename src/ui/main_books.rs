use leptos::{html::Input, *};
use leptos_router::{use_params, Route, A};

use crate::{server_api::User, ui::player::AudioProps};

#[component(transparent)]
pub fn MainBooks() -> impl IntoView {
    view! {
        <Route path="" view=BookIndex/>
        <Route path="book/:book_id" view=BookDetail/>
        <Route path="book/:book_id/chapter/:chapter_id" view=ChapterView/>
    }
}

#[component]
/// this will show all books
pub fn BookIndex() -> impl IntoView {
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
                                        {if !books.items.is_empty() {
                                            view! { <h1>{"Books:"}</h1> }
                                        } else {
                                            view! { <h1>{"No books"}</h1> }
                                        }}

                                        <div class="flex flex-row justify-between  items-center  space-y-1 space-x-1 flex-wrap w-full">
                                            {books
                                                .items
                                                .into_iter()
                                                .map(|(book, author)| {
                                                    view! {
                                                        <A
                                                            class="p-1  bg-blue-50 shadow-sm flex-auto hover:shadow-lg hover:bg-green-50"
                                                            href=format!("book/{}", book.id)
                                                        >

                                                            <h1>{&book.name}</h1>
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
pub fn BookDetail() -> impl IntoView {
    use leptos_router::Params;
    #[derive(Params, PartialEq, Clone)]
    struct Para {
        book_id: i32,
    }
    use crate::server_api::book::*;
    let params = use_params::<Para>();
    let set_player_props = use_context::<WriteSignal<Option<AudioProps>>>().unwrap();
    let (current_page, set_current_page) = create_signal(0u64);
    let page_node_ref: NodeRef<Input> = create_node_ref();
    let (max_item, _set_max_item) = create_signal(100);
    let user = use_context::<User>().unwrap();
    let book_author_chapters_detail = create_resource(
        move || {
            (
                params.get().unwrap().book_id,
                current_page.get(),
                max_item.get(),
            )
        },
        move |(book_id, current_page, max_item)| async move {
            let book_detail = get_book_detail(book_id).await.unwrap();
            let author_detail = crate::server_api::authors::get_author_by_id(book_detail.author_id)
                .await
                .unwrap()
                .unwrap();
            let chapters = get_chapters(book_detail.id, current_page, max_item)
                .await
                .unwrap();
            let current_p = get_progress(book_detail.id, user.id).await.unwrap();

            let progress = if let Some(p) = current_p {
                let chapter_detail = crate::server_api::book::get_chatper_detail(p.chapter_id)
                    .await
                    .unwrap();
                Some((p, chapter_detail))
            } else {
                None
            };
            (book_detail, author_detail, chapters, progress)
        },
    );

    use crate::server_api::progress::*;
    let user = use_context::<User>().unwrap();

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

            <Transition fallback=move || {
                view! { <p>{"Loading..."}</p> }
            }>
                <div class="flex-col flex items-center space-y-1 w-full">

                    {move || {
                        book_author_chapters_detail
                            .get()
                            .map(|(book, author, chapters, progress)| {
                                let number_of_pages = chapters.number_of_pages;
                                let current_page = chapters.page;
                                view! {
                                    <h1>{&book.name}</h1>
                                    <h2>{&author.name}</h2>

                                    {match progress {
                                        Some((progress, chapter)) => {
                                            let init_time = progress.progress;
                                            view! {
                                                <button
                                                    class="w-full mx-2 px-2 py-1 bg-blue-50 hover:bg-green-50 border border-solid rounded-sm shadow-md hover:shadow-lg"
                                                    on:click=move |_e| {
                                                        on_progress_button_click(user.id, book.id);
                                                    }
                                                >

                                                    <h2>{&book.name}</h2>

                                                    {move || {
                                                        let (min, sec) = crate::ui::translate_time(
                                                            init_time as i64,
                                                        );
                                                        let formated_time = crate::ui::formate_time(min, sec);
                                                        view! {
                                                            <h3>{&chapter.chapter_name}</h3>

                                                            <p>{format!("Current progress: {}", formated_time)}</p>
                                                        }
                                                            .into_view()
                                                    }}

                                                </button>
                                            }
                                                .into_view()
                                        }
                                        None => view! { <p>{"No Progress"}</p> }.into_view(),
                                    }}

                                    <div class="flex-col flex space-x-0 w-full my-1 py-1">

                                        {chapters
                                            .items
                                            .into_iter()
                                            .map(move |chapter| {
                                                let chapter_detail = chapter.clone();
                                                let chapter_name = chapter_detail.chapter_name;
                                                view! {
                                                    <A
                                                        class="w-full  px-2 py-1 bg-blue-50 hover:bg-green-50 border border-solid rounded-sm shadow-md hover:shadow-lg"
                                                        href=format!("chapter/{}", chapter.id)
                                                    >

                                                        {chapter_name}
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
                            })
                    }}

                </div>
            </Transition>

            <A
                class="w-full p-2 shadow-sm bg-blue-50 hover:bg-green-50 hover:shadow-xl rounded-md border border-solid"
                href="/books"
            >
                {"Back"}
            </A>
        </div>
    }
}

#[component]
pub fn ChapterView() -> impl IntoView {
    use leptos_router::*;
    #[derive(Params, PartialEq, Clone)]
    struct Para {
        book_id: i32,
        chapter_id: i32,
    }
    let params = use_params::<Para>();
    let set_player_props = use_context::<WriteSignal<Option<AudioProps>>>().unwrap();

    let book_chapter_detail = create_resource(
        move || {
            let para = params.get().unwrap();
            (para.book_id, para.chapter_id)
        },
        move |(book_id, chapter_id)| async move {
            let book_detail = crate::server_api::book::get_book_detail(book_id)
                .await
                .unwrap();
            let author_detail = crate::server_api::authors::get_author_by_id(book_detail.author_id)
                .await
                .unwrap()
                .unwrap();
            let chapter_detail = crate::server_api::book::get_chatper_detail(chapter_id)
                .await
                .unwrap();
            (book_detail, author_detail, chapter_detail)
        },
    );
    view! {
        <div class="flex flex-col  w-full">
            <Transition fallback=move || {
                view! { <p>{"Loading..."}</p> }
            }>
                {move || {
                    book_chapter_detail
                        .get()
                        .map(|(book_detail, _author_detail, chapter_detail)| {
                            let book_name = book_detail.name.clone();
                            view! {
                                <div class="flex flex-col items-stretch w-full space-y-2">
                                    <h1>{&book_name}</h1>
                                    <h2>{&chapter_detail.chapter_name}</h2>
                                    <button
                                        class="w-full p-2 shadow-sm bg-blue-50 hover:bg-green-50 hover:shadow-xl rounded-md border border-solid"
                                        on:click=move |_| {
                                            set_player_props(
                                                Some(AudioProps {
                                                    book_id: book_detail.id,
                                                    chapter_id: chapter_detail.id,
                                                    init_time: 0.,
                                                }),
                                            );
                                        }
                                    >

                                        start Play
                                    </button>
                                    <button

                                        class="w-full p-2 shadow-sm bg-blue-50 hover:bg-green-50 hover:shadow-xl rounded-md border border-solid"
                                        on:click=move |_| {
                                            set_player_props(None);
                                        }
                                    >

                                        Stop Play
                                    </button>
                                    <A
                                        class="w-full p-2 shadow-sm bg-blue-50 hover:bg-green-50 hover:shadow-xl rounded-md border border-solid"
                                        href=format!("/books/book/{}", book_detail.id)
                                    >

                                        Back
                                    </A>
                                </div>
                            }
                        })
                }}

            </Transition>
        </div>
    }
}
