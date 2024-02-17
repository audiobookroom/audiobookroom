use leptos::*;
#[component]

pub fn DeleteUser() -> impl IntoView {
    let delete_user_action = create_server_action::<crate::server_api::auth::DeleteUser>();
    let self_id = use_context::<crate::server_api::User>().unwrap().id;
    let user_list = create_resource(move ||{
        delete_user_action.version().get()
    }, move |_| async move {
        let users = crate::server_api::auth::get_all_users().await;
        users
    });
    view! {
        <h2>{"Delete User"}</h2>
        <div class="flex flex-col w-full space-y-1 p-2">
            // the users
            <div class="flex flex-row w-full space-x-2 p-1 justify-between items-center flex-wrap">
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
                                                        delete_user_action
                                                            .dispatch(crate::server_api::auth::DeleteUser {
                                                                user_id: user.id,
                                                            });
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

            </div>

            <p>
                {move || {
                    delete_user_action
                        .value()
                        .get()
                        .map(|result| match result {
                            Ok(_) => "User deleted".to_string(),
                            Err(e) => e.to_string(),
                        })
                }}

            </p>
        </div>
    }
}
