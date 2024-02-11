use leptos::*;

use crate::server_api::progress::ProgressResult;

#[component]
pub fn MainIndex() -> impl IntoView {
    use crate::server_api::auth::User;

    let user = use_context::<User>().unwrap();
    let set_player_prop =
        use_context::<WriteSignal<Option<crate::ui::player::AudioProps>>>().unwrap();

    let current_progress = create_resource(
        move || {},
        move |_| async move {
            let current_p = crate::server_api::progress::get_progress_detail_by_user(user.id)
                .await
                .unwrap();
            current_p
        },
    );
    let global_current_progress = use_context::<RwSignal<Option<ProgressResult>>>().unwrap();
    view! {
        <div class="flex flex-col items-center text-center w-full">
            <Transition fallback=move || {
                view! { <span>"Loading..."</span> }
            }>
                <h1>{"Welcome to the library"}</h1>
                {move || {
                    current_progress
                        .get()
                        .map(|p| {
                            view! {
                                {if !p.is_empty() {
                                    view! { <h1>{"Continue reading"}</h1> }
                                } else {
                                    view! { <h1>{"No recent record."}</h1> }
                                }}

                                {move || {
                                    p.iter()
                                        .map(|(progress_item, book, chapter)| {
                                            let book_id = progress_item.music_id;
                                            let chapter_id = progress_item.chapter_id;
                                            let init_time = progress_item.progress;
                                            let total_chapters = book.chapters;
                                            let chapter_name = chapter.chapter_name.clone();
                                            view! {
                                                <button
                                                    class="w-full mx-2 px-2 py-1 bg-blue-50 hover:bg-green-50 border border-solid rounded-sm shadow-md hover:shadow-lg"
                                                    on:click=move |_e| {
                                                        if let Some(global_prgress) = global_current_progress.get()
                                                            && global_prgress.music_id == book_id
                                                        {
                                                            set_player_prop(
                                                                Some(crate::ui::player::AudioProps {
                                                                    book_id,
                                                                    chapter_id: global_prgress.chapter_id,
                                                                    init_time: global_prgress.progress,
                                                                    total_chapters,
                                                                }),
                                                            );
                                                        } else {
                                                            set_player_prop(
                                                                Some(crate::ui::player::AudioProps {
                                                                    book_id,
                                                                    chapter_id,
                                                                    init_time,
                                                                    total_chapters,
                                                                }),
                                                            );
                                                        }
                                                    }
                                                >

                                                    <h2>{&book.name}</h2>
                                                    {move || {
                                                        if let Some(global_prgress) = global_current_progress.get()
                                                            && global_prgress.music_id == book_id
                                                        {
                                                            let time = global_prgress.progress;
                                                            return view! {
                                                                <span>{format!("Continue: {}", time)}</span>
                                                            }
                                                                .into_view();
                                                        } else {
                                                            return view! {
                                                                <h3>{&chapter_name}</h3>
                                                                <p>{format!("Current progress: {}", init_time)}</p>
                                                            }
                                                                .into_view();
                                                        }
                                                    }}

                                                </button>
                                            }
                                                .into_view()
                                        })
                                        .collect_view()
                                }}
                            }
                        })
                }}

            </Transition>

        </div>
    }
}
