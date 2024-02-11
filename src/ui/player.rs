use crate::server_api::{
    auth::User,
    book::{get_book_detail, get_chatper_detail},
};
use leptos::{ev::MouseEvent, html::Audio, *};
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AudioProps {
    pub chapter_id: i32,
    pub book_id: i32,
    pub total_chapters: i32,
    pub init_time: f64,
}
#[component]
pub fn Player(
    props: ReadSignal<Option<AudioProps>>,
    #[prop(into)] on_ended: Callback<AudioProps, ()>,
    #[prop(into)] on_previouse: Callback<AudioProps, ()>,
    #[prop(into)] on_next: Callback<AudioProps, ()>,
) -> impl IntoView {
    // will not play if the props is not changed
    use crate::server_api::progress::*;
    let props = create_memo(move |_| props.get());
    let player_ref_node: NodeRef<Audio> = create_node_ref();
    let (last_saved_time, set_last_saved_time) = create_signal(None::<SetProgress>);
    let (current_time, set_current_time) = create_signal(0.0);
    let (total_time, set_total_time) = create_signal(0.0);
    let user = use_context::<User>().unwrap();
    let set_progress_action =
        use_context::<Action<SetProgress, Result<(), ServerFnError>>>().unwrap();
    // fetch the detail from server
    let play_details = create_resource(
        move || props.get(),
        move |props| async move {
            if let Some(p) = props {
                Some((
                    get_book_detail(p.book_id).await.unwrap(),
                    get_chatper_detail(p.chapter_id).await.unwrap(),
                    p.init_time,
                ))
            } else {
                None
            }
        },
    );
    let on_duration_change = move |_e| {
        let total_time = player_ref_node.get().unwrap().duration();
        set_total_time(total_time);
    };
    let global_progress = use_context::<RwSignal<Option<ProgressResult>>>().unwrap();
    let on_time_updated = move |_e| {
        // first set current time

        let current_time = player_ref_node.get().unwrap().current_time();
        set_current_time(current_time);
        let last_saved_time = last_saved_time.get();
        let props = props.get();

        if let Some(props) = props {
            global_progress.set(Some(ProgressResult {
                account_id: user.id,
                music_id: props.book_id,
                chapter_id: props.chapter_id,
                progress: current_time,
            }));
            if let Some(last_save_time) = last_saved_time {
                if last_save_time.chapter_id != props.chapter_id
                    || (last_save_time.progress - current_time).abs() >= 10.
                {
                    let new_set_progress = SetProgress {
                        account_id: user.id,
                        music_id: props.book_id,
                        chapter_id: props.chapter_id,
                        progress: current_time,
                    };
                    set_last_saved_time(Some(new_set_progress.clone()));
                    set_progress_action.dispatch(new_set_progress);
                }
            } else {
                let new_set_progress = SetProgress {
                    account_id: user.id,
                    music_id: props.book_id,
                    chapter_id: props.chapter_id,
                    progress: current_time,
                };
                set_last_saved_time(Some(new_set_progress.clone()));
                set_progress_action.dispatch(new_set_progress);
            }
        }
    };
    let (current_playing, set_current_playing) = create_signal(false);
    let on_player_pause_pushed = move |_| {
        let player = player_ref_node.get().unwrap();
        if player.paused() {
            let _ = player.play().unwrap();
        } else {
            player.pause().unwrap();
        }
    };
    let on_select_position = move |e: MouseEvent| {
        let player = player_ref_node.get().unwrap();
        let target = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("progress_bar")
            .unwrap();
        let rect = target.get_bounding_client_rect();
        let x = e.client_x() as f64 - rect.left();
        let width = rect.width();
        let percent = x / width;
        let time = player.duration() * percent;
        player.set_current_time(time);
    };
    view! {
        <div>
            <Transition fallback=move || {
                view! { <span>"Loading..."</span> }
            }>
                {move || {
                    play_details
                        .get()
                        .map(|p| {
                            match p {
                                Some(p) => {
                                    let (book_detail, chapter_detail, init_time) = p;
                                    let url = format!("/fetchbook/{}", chapter_detail.chapter_url);
                                    view! {
                                        <audio
                                            style="display: none"
                                            src=url
                                            autoplay=true
                                            prop:currentTime=init_time
                                            on:ended=move |_e| on_ended(AudioProps {
                                                chapter_id: chapter_detail.id,
                                                book_id: book_detail.id,
                                                total_chapters: book_detail.chapters,
                                                init_time: 0.0,
                                            })

                                            on:durationchange=on_duration_change

                                            on:timeupdate=on_time_updated
                                            on:play=move |_| set_current_playing(true)
                                            on:pause=move |_| set_current_playing(false)
                                            ref=player_ref_node
                                        ></audio>

                                        <div class="w-full">
                                            <div class="h-2 bg-red-light"></div>
                                            <div class="flex items-center justify-center bg-red-lightest">
                                                <div
                                                    class="bg-white shadow-lg rounded-lg"
                                                    style="width: 45rem !important"
                                                >
                                                    <div class="flex">
                                                        <div>
                                                            <img
                                                                class="w-full rounded hidden md:block"
                                                                src="/icon.png"
                                                                alt="Album Pic"
                                                            />
                                                        </div>
                                                        <div class="w-full p-2">
                                                            <div class="flex justify-between">
                                                                <div>
                                                                    <h3 class="text-lg text-grey-darkest font-medium">

                                                                        {book_detail.name}

                                                                    </h3>
                                                                    <p class="text-sm text-grey mt-1">

                                                                        {chapter_detail.chapter_name}

                                                                    </p>
                                                                </div>
                                                               
                                                            </div>
                                                            <div class="flex justify-between items-center mt-2">
                                                              
                                                                <div
                                                                    class="text-grey-darker hover:shadow-lg hover:bg-slate-100"
                                                                    on:click=move |_| on_previouse(AudioProps {
                                                                        chapter_id: chapter_detail.id,
                                                                        book_id: book_detail.id,
                                                                        total_chapters: book_detail.chapters,
                                                                        init_time: 0.0,
                                                                    })
                                                                >

                                                                    <svg
                                                                        class="w-8 h-8"
                                                                        fill="currentColor"
                                                                        xmlns="http://www.w3.org/2000/svg"
                                                                        viewBox="0 0 20 20"
                                                                    >
                                                                        <path d="M4 5h3v10H4V5zm12 0v10l-9-5 9-5z"></path>
                                                                    </svg>
                                                                </div>
                                                                <div
                                                                    class="text-grey-darker p-8 rounded-full bg-red-light shadow-lg hover:shadow-2xl hover:bg-slate-100"
                                                                    on:click=on_player_pause_pushed
                                                                >

                                                                    {move || {
                                                                        if current_playing.get() {
                                                                            view! {
                                                                                <svg
                                                                                    class="w-8 h-8"
                                                                                    fill="currentColor"
                                                                                    xmlns="http://www.w3.org/2000/svg"
                                                                                    viewBox="0 0 20 20"
                                                                                >
                                                                                    <path d="M5 4h3v12H5V4zm7 0h3v12h-3V4z"></path>
                                                                                </svg>
                                                                            }
                                                                        } else {
                                                                            view! {
                                                                                <svg
                                                                                    class="w-8 h-8"
                                                                                    fill="currentColor"
                                                                                    xmlns="http://www.w3.org/2000/svg"
                                                                                    viewBox="0 0 20 20"
                                                                                >
                                                                                    <path d="M5 4v12l10-6z"></path>
                                                                                </svg>
                                                                            }
                                                                        }
                                                                    }}

                                                                </div>
                                                                <div
                                                                    class="text-grey-darker hover:shadow-lg hover:bg-slate-100"
                                                                    on:click=move |_| on_next(AudioProps {
                                                                        chapter_id: chapter_detail.id,
                                                                        book_id: book_detail.id,
                                                                        total_chapters: book_detail.chapters,
                                                                        init_time: 0.0,
                                                                    })
                                                                >

                                                                    <svg
                                                                        class="w-8 h-8"
                                                                        fill="currentColor"
                                                                        xmlns="http://www.w3.org/2000/svg"
                                                                        viewBox="0 0 20 20"
                                                                    >
                                                                        <path d="M13 5h3v10h-3V5zM4 5l9 5-9 5V5z"></path>
                                                                    </svg>
                                                                </div>
                                                                
                                                            </div>
                                                        </div>
                                                    </div>
                                                    <div class="mx-8 py-4">
                                                        <div class="flex justify-between text-sm text-grey-darker">
                                                            <p>
                                                                {move || {
                                                                    let current_time = current_time.get();
                                                                    format!(
                                                                        "{:02}:{:02}",
                                                                        current_time as i32 / 60,
                                                                        current_time as i32 % 60,
                                                                    )
                                                                }}

                                                            </p>
                                                            <p>
                                                                {move || {
                                                                    let total_time = total_time.get();
                                                                    format!(
                                                                        "{:02}:{:02}",
                                                                        total_time as i32 / 60,
                                                                        total_time as i32 % 60,
                                                                    )
                                                                }}

                                                            </p>
                                                        </div>
                                                        <div
                                                            class="group mt-1 h-4 flex justify-center items-center"
                                                            id="progress_bar"
                                                            on:click=on_select_position
                                                        >
                                                            <div class=" bg-red-100 group-hover:bg-red-200 group-hover:shadow-lg w-full h-2  border rounded-full">
                                                                <div
                                                                    class="bg-red-400 h-full border-solid rounded-full relative"
                                                                    style:width=move || {
                                                                        format!(
                                                                            "{}%",
                                                                            current_time.get() / total_time.get() * 100.0,
                                                                        )
                                                                    }
                                                                >
                                                                </div>
                                                                <div
                                                                    class="w-4 h-4 bg-red-400 pin-r pin-b -mt-1 rounded-full shadow bottom-2 relative"
                                                                    style:left=move || {
                                                                        format!(
                                                                            "{}%",
                                                                            current_time.get() / total_time.get() * 100.0,
                                                                        )
                                                                    }
                                                                >
                                                                </div>
                                                            </div>
                                                        </div>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                    }
                                        .into_view()
                                }
                                None => {
                                    view! {
                                        <div>"No audio set"</div>
                                    }
                                        .into_view()
                                }
                            }
                        })
                }}

            </Transition>

        </div>
    }
}
