use leptos::*;
use leptos_router::ActionForm;

use crate::{
    server_api::{
        auth::{Logout, User},
        progress::{ProgressResult, SetProgress},
    },
    ui::{
        main_authors::MainAuthors,
        main_books::MainBooks,
        main_index::MainIndex,
        main_setting::MainSettings,
        player::{AudioProps, Player},
    },
};

use super::main_books::BookPageContent;

#[derive(Clone, Debug, PartialEq)]
pub enum MainPageContent {
    Index,
    Books,
    Authors,
    Settings,
}
#[component]
pub fn MainPage(user: User, logout: super::ServerAction<Logout, ()>) -> impl IntoView {
    let (current_content, set_current_content) = create_signal(MainPageContent::Index);
    let (book_current_content, set_book_current_content) = create_signal(BookPageContent::Index);

    let (player_status, set_player_status) = create_signal("".to_string());
    let (player_props, set_player_props) = create_signal(None);
    let player_on_previouse = move |mut prop: AudioProps| {
        spawn_local(async move {
            let chapter_detail = crate::server_api::book::get_chatper_detail(prop.chapter_id)
                .await
                .unwrap();
            if chapter_detail.chapter_num > 0 {
                // next chapter
                // fetch the next chapter
                let next_chapter_detail: crate::server_api::book::ChapterDetail =
                    crate::server_api::book::search_chapter_by_chapter_num(
                        prop.book_id,
                        chapter_detail.chapter_num - 1,
                    )
                    .await
                    .unwrap();
                prop.chapter_id = next_chapter_detail.id;
                prop.init_time = 0.;
                set_player_props(Some(prop));
            } else {
                // end it
                set_player_status("End".to_string());
                set_player_props(None);
            }
        })
    };
    let player_on_ended = move |mut prop: AudioProps| {
        spawn_local(async move {
            let book_detail = crate::server_api::book::get_book_detail(prop.book_id)
                .await
                .unwrap();
            let chapter_detail = crate::server_api::book::get_chatper_detail(prop.chapter_id)
                .await
                .unwrap();
            if chapter_detail.chapter_num < book_detail.chapters - 1 {
                // next chapter
                // fetch the next chapter
                let next_chapter_detail: crate::server_api::book::ChapterDetail =
                    crate::server_api::book::search_chapter_by_chapter_num(
                        prop.book_id,
                        chapter_detail.chapter_num + 1,
                    )
                    .await
                    .unwrap();
                prop.chapter_id = next_chapter_detail.id;
                prop.init_time = 0.;
                set_player_props(Some(prop));
            } else {
                // end it
                set_player_status("End".to_string());
                set_player_props(None);
            }
        })
    };
    let set_progress_action: Action<SetProgress, Result<(), ServerFnError>> =
        create_server_action::<SetProgress>();

    // send the progress to server
    provide_context(set_progress_action);
    // set current play info
    provide_context(set_player_props);
    // set current user
    provide_context(user.clone());
    let global_progress_signal = create_rw_signal(Option::<ProgressResult>::None);
    provide_context(global_progress_signal);

    view! {
        <div class="flex flex-col justify-between items-stretch h-full max-h-full p-2 lg:p-4  mx-auto overflow-hidden max-w-lg ">
            <div class="flex flex-col my-2">
                <div class="flex justify-between items-center">
                    <span>{format!("Welcome {}", user.username)}</span>
                    <ActionForm action=logout>
                        <button
                            type="submit"
                            class="  hover:bg-gray-50 hover:shadow-lg text-gray-800"
                        >
                            Log Out
                        </button>
                    </ActionForm>
                </div>
                <div class="flex justify-between items-center space-x-2">
                    <button
                        class="flex-1 bg-gray-400 shadow-md hover:bg-gray-50 hover:shadow-lg"
                        on:click=move |_| set_current_content(MainPageContent::Index)
                    >
                        "Index"
                    </button>
                    <button
                        class="flex-1 bg-gray-400 shadow-md hover:bg-gray-50 hover:shadow-lg"
                        on:click=move |_| {
                            set_current_content(MainPageContent::Books);
                            set_book_current_content(BookPageContent::Index);
                        }
                    >

                        "Books"
                    </button>
                    <button
                        class="flex-1 bg-gray-400 shadow-md hover:bg-gray-50 hover:shadow-lg"
                        on:click=move |_| set_current_content(MainPageContent::Authors)
                    >
                        "Authors"
                    </button>
                    <button
                        class="flex-1 bg-gray-400 shadow-md hover:bg-gray-50 hover:shadow-lg"
                        on:click=move |_| set_current_content(MainPageContent::Settings)
                    >
                        "Settings"
                    </button>
                </div>
            </div>
            <div class="flex-1 overflow-auto flex flex-col items-center text-center">

                {move || match current_content.get() {
                    MainPageContent::Index => view! { <MainIndex/> },
                    MainPageContent::Books => {
                        view! {
                            <MainBooks
                                current_content=book_current_content
                                set_current_content=set_book_current_content
                            />
                        }
                    }
                    MainPageContent::Authors => {
                        view! {
                            <MainAuthors
                                set_main_content=set_current_content
                                set_book_current_content=set_book_current_content
                            />
                        }
                    }
                    MainPageContent::Settings => view! { <MainSettings/> },
                }}

            </div>
            <div>
                <Player
                    props=player_props
                    on_ended=player_on_ended
                    on_next=player_on_ended
                    on_previouse=player_on_previouse
                />
                <span>{player_status}</span>

            </div>
        </div>
    }
}
