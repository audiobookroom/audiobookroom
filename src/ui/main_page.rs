use leptos::{ *};
use leptos_router::{Outlet, A};

use crate::{
    server_api::{progress::SetProgress, User},
    ui::player::{AudioProps, Player},
};

#[derive(Clone, Copy, Debug)]
pub struct RefreshSignal;

// everytime update RefreshSignal, should make it inequal
impl PartialEq for RefreshSignal {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

#[component]
pub fn MainPage(user: User) -> impl IntoView {
    let refresh_signle = create_rw_signal(RefreshSignal);

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
                // save the progress
                let _ = crate::server_api::progress::set_progress(
                    user.id,
                    prop.book_id,
                    prop.chapter_id,
                    0.,
                )
                .await;
                refresh_signle.set(RefreshSignal);
            } else {
                // end it
                set_player_status("End".to_string());
                set_player_props(None);
                refresh_signle.set(RefreshSignal);
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
                // save the progress
                let _ = crate::server_api::progress::set_progress(
                    user.id,
                    prop.book_id,
                    prop.chapter_id,
                    0.,
                )
                .await;
                refresh_signle.set(RefreshSignal);
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
    provide_context(refresh_signle);
    view! {
        <div class="flex flex-col justify-between items-stretch h-full max-h-full p-2 lg:p-4  mx-auto overflow-hidden max-w-lg ">
            <div class="flex flex-col my-2">
                <div class="flex justify-between items-center">
                    <span>{format!("Welcome {}", user.username)}</span>

                </div>

                <div class="flex justify-between items-center space-x-2">
                    <A
                        class="flex-1 bg-gray-400 shadow-md hover:bg-gray-50 hover:shadow-lg"
                        href="/"
                    >
                        "Index"
                    </A>
                    <A
                        class="flex-1 bg-gray-400 shadow-md hover:bg-gray-50 hover:shadow-lg"
                        href="/books"
                    >

                        "Books"
                    </A>
                    <A
                        class="flex-1 bg-gray-400 shadow-md hover:bg-gray-50 hover:shadow-lg"
                        href="/authors"
                    >
                        "Authors"
                    </A>
                    <A
                        class="flex-1 bg-gray-400 shadow-md hover:bg-gray-50 hover:shadow-lg"
                        href="/settings"
                    >
                        "Settings"
                    </A>
                </div>
            </div>
            <div class="flex-1 overflow-auto flex flex-col items-center text-center">

                <Outlet/>

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
