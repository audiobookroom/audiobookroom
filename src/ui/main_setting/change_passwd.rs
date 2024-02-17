use leptos::{html::Input, *};

#[component]
pub fn ChangePassword()->impl IntoView{
    let user = use_context::<crate::server_api::User>().unwrap();
    let password_node = create_node_ref::<Input>();
    let change_password = create_server_action::<crate::server_api::auth::ChangeUserPasswd>();
    let on_change_password = move |_| {
        let password = password_node.get().unwrap().value();
        change_password.dispatch(crate::server_api::auth::ChangeUserPasswd {
            user_id: user.id,
            new_password: password,
        });
        password_node.get().unwrap().set_value("");
    };
    let result = change_password.value();
    view! {
        <div class="flex flex-col w-full space-y-1 p-2">
            <h2>{"Change Password"}</h2>
            <input
                class="w-full  my-1 px-4 py-1 bg-gray-100 hover:bg-gray-50 hover:shadow-lg border border-solid rounded-full"
                type="password"
                ref=password_node
                placeholder="New Password"
            />
            <button
                class="w-full   px-1 py-1  bg-gray-400 hover:bg-gray-50 hover:shadow-lg border border-solid rounded-full"
                on:click=on_change_password
            >
                {"Change Password"}
            </button>
        </div>
        <p>
            {move || {
                result
                    .get()
                    .map(|result| match result {
                        Ok(_) => "Password changed".to_string(),
                        Err(e) => e.to_string(),
                    })
            }}

        </p>
    }
}