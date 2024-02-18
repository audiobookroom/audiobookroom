use leptos::*;
use tracing::info;

#[derive(Clone)]
enum DeleteUserContent {
    Main,
    DeleteUserDetail(i32),
}

#[component]
pub fn DeleteUser() -> impl IntoView {
    let (current_content, set_current_content) = create_signal(DeleteUserContent::Main);
    let self_id = use_context::<crate::server_api::User>().unwrap().id;
    let delete_user_action: Action<crate::server_api::auth::DeleteUser, Result<(), ServerFnError>> =
        create_server_action::<crate::server_api::auth::DeleteUser>();

    let user_list = create_resource(
        move || delete_user_action.version().get(),
        move |_| async move {
            let users = crate::server_api::auth::get_all_users().await;
            users
        },
    );
    view! {
        {move || {
            match current_content.get() {
                DeleteUserContent::Main => {
                    view! {
                        <h2>{"Delete User"}</h2>
                        <div class="flex flex-col w-full space-y-1 p-2">
                            <div class="flex flex-row w-full space-x-2 p-1 justify-between items-center flex-wrap">
                                <Transition fallback=move || {
                                    view! { <span>"Loading..."</span> }
                                }>
                                    {move || {
                                        user_list
                                            .get()
                                            .map(|users| {
                                                match users {
                                                    Ok(users) => {
                                                        users
                                                            .into_iter()
                                                            .map(|user| {
                                                                view! {
                                                                    <button
                                                                        class="flex-1 text-center bg-blue-50 hover:bg-green-50 px-1 py-1 my-2 border border-solid rounded-full disabled:opacity-50 disabled:bg-gray-200"
                                                                        on:click=move |_| {
                                                                            set_current_content(
                                                                                DeleteUserContent::DeleteUserDetail(user.id),
                                                                            );
                                                                            info!("User: {}", user.id);
                                                                        }

                                                                        prop:disabled=move || { user.id == self_id }
                                                                    >
                                                                        <p>
                                                                            {if user.id == self_id {
                                                                                format!("You {}", user.username)
                                                                            } else {
                                                                                user.username
                                                                            }}

                                                                        </p>
                                                                    </button>
                                                                }
                                                            })
                                                            .collect_view()
                                                    }
                                                    Err(e) => view! { <p>{e.to_string()}</p> }.into_view(),
                                                }
                                            })
                                    }}

                                </Transition>
                            </div>

                        </div>
                    }
                        .into_view()
                }
                DeleteUserContent::DeleteUserDetail(user_id) => {
                    view! {
                        <DeleteUserDetail
                            user_id=user_id
                            set_current_content=set_current_content
                            delete_user_action=delete_user_action
                        />
                    }
                        .into_view()
                }
            }
        }}
    }
}

#[component]
fn DeleteUserDetail(
    user_id: i32,
    set_current_content: WriteSignal<DeleteUserContent>,
    delete_user_action: Action<crate::server_api::auth::DeleteUser, Result<(), ServerFnError>>,
) -> impl IntoView {
    let user: Resource<(), Result<crate::server_api::User, ServerFnError>> = create_resource(
        || {},
        move |_| async move {
            use crate::server_api::auth::get_user_by_id;
            let user = get_user_by_id(user_id).await;
            user
        },
    );
    let (button_enabled, set_button_enabled) = create_signal(true);
    let deltte_user_on_click = move |_| {
        delete_user_action.dispatch(crate::server_api::auth::DeleteUser { user_id: user_id });
        set_button_enabled(false);
    };
    let delete_value = delete_user_action.value();
    view! {
        <div class="flex flex-col w-full space-y-1 p-2">
            <h2>{"Delete User"}</h2>
            <div class="flex flex-col w-full space-y-1 p-2">
                <Transition fallback=move || {
                    view! { <span>"Loading..."</span> }
                }>

                    {move || {
                        user.get()
                            .map(move |user| {
                                match user {
                                    Ok(user) => {
                                        view! {
                                            <p>{format!("Delete: User: {} ?", user.username)}</p>
                                            <button
                                                class="bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded disabled:bg-gray-400"
                                                on:click=deltte_user_on_click
                                                prop:disabled=move || { !button_enabled.get() }
                                            >
                                                {"Delete"}
                                            </button>
                                            <button
                                                class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
                                                on:click=move |_| {
                                                    set_current_content(DeleteUserContent::Main);
                                                }
                                            >

                                                {"Back"}
                                            </button>
                                            <p>
                                                {move || {
                                                    delete_value
                                                        .get()
                                                        .map(|result| {
                                                            match result {
                                                                Ok(_) => "User deleted".to_string(),
                                                                Err(e) => e.to_string(),
                                                            }
                                                        })
                                                }}

                                            </p>
                                        }
                                            .into_view()
                                    }
                                    Err(e) => view! { <p>{e.to_string()}</p> }.into_view(),
                                }
                            })
                    }}

                </Transition>
            </div>

        </div>
    }
}
