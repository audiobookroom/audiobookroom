use leptos::*;

use crate::server_api::User;

#[component]
pub fn MainIndex() -> impl IntoView {
    let user = use_context::<User>().unwrap();
    let set_player_prop =
        use_context::<WriteSignal<Option<crate::ui::player::AudioProps>>>().unwrap();
    let refresh_signle: RwSignal<crate::ui::main_page::RefreshSignal> = use_context().unwrap();
    let current_progress = create_resource(
        move || refresh_signle.get(),
        move |_| async move {
            let mut current_p = crate::server_api::progress::get_progress_detail_by_user(user.id)
                .await
                .unwrap();
            current_p.sort_unstable_by(|a, b| b.0.update.cmp(&a.0.update));
            current_p
        },
    );
    let on_progress_button_click = move |account_id: i32, book_id: i32| {
        spawn_local(async move {
            let progress = crate::server_api::progress::get_progress(book_id, account_id).await;
            if let Ok(Some(k)) = progress {
                set_player_prop(Some(crate::ui::player::AudioProps {
                    book_id,
                    chapter_id: k.chapter_id,
                    init_time: k.progress,
                }));
                // refresh the signle
                refresh_signle.set(crate::ui::main_page::RefreshSignal);
            }
        });
    };

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
                                            let init_time = progress_item.progress;
                                            let chapter_name = chapter.chapter_name.clone();
                                            let (min, sec) = super::translate_time(init_time as i64);
                                            let last_update = progress_item.update;
                                            let local_update = last_update
                                                .with_timezone(&chrono::Local);
                                            let local_str = local_update
                                                .format("%Y-%m-%d %H:%M:%S")
                                                .to_string();
                                            view! {
                                                <button
                                                    class="w-full mx-2 px-2 py-1 bg-blue-50 hover:bg-green-50 border border-solid rounded-sm shadow-md hover:shadow-lg"
                                                    on:click=move |_e| {
                                                        on_progress_button_click(user.id, book_id);
                                                    }
                                                >

                                                    <h2>{&book.name}</h2>
                                                    {move || {
                                                        view! {
                                                            <h3>{&chapter_name}</h3>
                                                            <p>
                                                                {format!(
                                                                    "Current progress: {}",
                                                                    super::formate_time(min, sec),
                                                                )}

                                                            </p>
                                                            <p>{format!("Last update: {}", local_str)}</p>
                                                        }
                                                            .into_view()
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
