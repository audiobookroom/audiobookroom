use leptos::{html::Input, *};
#[component]
pub fn AddUser() -> impl IntoView {
    use crate::server_api::auth::*;
    let user_name_node = create_node_ref::<Input>();
    let password_node = create_node_ref::<Input>();
    let role_node = create_node_ref::<Input>();
    let create_user_action = create_server_action::<AddUser>();
    let action_result = create_user_action.value();
    let (error_msg, set_error_msg) = create_signal(String::new());
    let on_create_user = move |_| {
        let username = user_name_node.get().unwrap().value();
        let password = password_node.get().unwrap().value();
        let role = role_node.get().unwrap().value().parse::<i32>().unwrap();

        if role != 0 && role != 1 {
            set_error_msg("Role must be 0 or 1".to_string());
            return;
        }
        create_user_action.dispatch(AddUser {
            username,
            password,
            role,
        });
        // clear the input
        user_name_node.get().unwrap().set_value("");
        password_node.get().unwrap().set_value("");
        role_node.get().unwrap().set_value("");
    };
    view! {
        <div class="flex flex-col w-full space-y-1 p-2">
            <h2>{"Add User"}</h2>
            <input
                class="w-full  my-1 px-4 py-1 bg-gray-100 hover:bg-gray-50 hover:shadow-lg border border-solid rounded-full"
                type="text"
                ref=user_name_node
                placeholder="User Name"
            />
            <input
                class="w-full  my-1 px-4 py-1 bg-gray-100 hover:bg-gray-50 hover:shadow-lg border border-solid rounded-full"
                type="password"
                ref=password_node
                placeholder="Password"
            />
            <input
                class="w-full  my-1 px-4 py-1 bg-gray-100 hover:bg-gray-50 hover:shadow-lg border border-solid rounded-full"
                type="number"
                ref=role_node
                placeholder="Role 0 for admin, 1 for user"
            />
            <button
                class="w-full   px-1 py-1  bg-gray-400 hover:bg-gray-50 hover:shadow-lg border border-solid rounded-full"
                on:click=on_create_user
            >
                {"Create User"}
            </button>
            <p>{error_msg}</p>
            <p>
                {move || {
                    action_result
                        .get()
                        .map(|r| {
                            match r {
                                Ok(_r) => format!("User  created"),
                                Err(e) => format!("Error: {}", e),
                            }
                        })
                }}

            </p>
        </div>
    }
}
