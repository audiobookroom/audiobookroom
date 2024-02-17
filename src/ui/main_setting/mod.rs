use leptos::*;

use crate::{server_api::auth::Logout, ui::app::LogoutContext};

mod add_book;
mod add_user;
mod change_passwd;
mod del_book;
mod del_user;

#[derive(Clone, Debug, PartialEq)]
pub enum SettingsContent {
    Main,
    AddBook,
    DeleteBook,
    DeleteBookDetail(i32),
    AddUser,
    DeleteUser,
    ChangePassword,
}

#[component]
pub fn MainSettings() -> impl IntoView {
    let (current_content, set_current_content) = create_signal(SettingsContent::Main);
    let user = use_context::<crate::server_api::User>().unwrap();
    let logout_action = use_context::<LogoutContext>().unwrap().0;
    view! {
        {move || {
            match current_content.get() {
                SettingsContent::Main => {
                    view! {
                        <div class="flex flex-col items-center w-full">
                            <h1>{"Settings"}</h1>
                            <button
                                class="w-full text-center bg-blue-50 hover:bg-green-50 px-1 py-1 my-2 border border-solid rounded-full disabled:opacity-50 disabled:bg-gray-200"
                                prop:disabled=move || { user.role != 0 }
                                on:click=move |_| {
                                    if user.role == 0 {
                                        set_current_content(SettingsContent::AddBook)
                                    }
                                }
                            >

                                {if user.role == 0 {
                                    "Add Book"
                                } else {
                                    "Add Book(no permission)"
                                }}

                            </button>
                            <button
                                class="w-full text-center bg-blue-50 hover:bg-green-50 px-1 py-1 my-2 border border-solid rounded-full disabled:opacity-50 disabled:bg-gray-200"
                                prop:disabled=move || { user.role != 0 }
                                on:click=move |_| {
                                    if user.role == 0 {
                                        set_current_content(SettingsContent::DeleteBook)
                                    }
                                }
                            >

                                {if user.role == 0 {
                                    "Delete Book"
                                } else {
                                    "Delete Book(no permission)"
                                }}

                            </button>
                            <button
                                class="w-full text-center bg-blue-50 hover:bg-green-50 px-1 py-1 my-2 border border-solid rounded-full disabled:opacity-50 disabled:bg-gray-200"
                                prop:disabled=move || { user.role != 0 }
                                on:click=move |_| {
                                    if user.role == 0 {
                                        set_current_content(SettingsContent::AddUser)
                                    }
                                }
                            >

                                {if user.role == 0 {
                                    "Add User"
                                } else {
                                    "Add User(no permission)"
                                }}

                            </button>
                            <button
                                class="w-full text-center bg-blue-50 hover:bg-green-50 px-1 py-1 my-2 border border-solid rounded-full disabled:opacity-50 disabled:bg-gray-200"
                                prop:disabled=move || { user.role != 0 }
                                on:click=move |_| {
                                    if user.role == 0 {
                                        set_current_content(SettingsContent::DeleteUser)
                                    }
                                }
                            >

                                {if user.role == 0 {
                                    "Delete User"
                                } else {
                                    "Delete User(no permission)"
                                }}

                            </button>

                            <button
                                class="w-full text-center bg-blue-50 hover:bg-green-50 px-1 py-1 my-2 border border-solid rounded-full"
                                on:click=move |_| {
                                    set_current_content(SettingsContent::ChangePassword)
                                }
                            >

                                {"Change Password"}

                            </button>
                            <button
                                class="w-full text-center bg-red-200 hover:bg-red-300 px-1 py-1 my-2 border border-solid rounded-full"
                                on:click=move |_| {
                                    logout_action.dispatch(Logout {});
                                }
                            >

                                {"Logout"}

                            </button>

                        </div>
                    }
                        .into_view()
                }
                SettingsContent::AddBook => {
                    view! { <add_book::AddBook></add_book::AddBook> }.into_view()
                }
                SettingsContent::DeleteBook => {
                    view! {
                        <del_book::DeleteBook set_content=set_current_content></del_book::DeleteBook>
                    }
                        .into_view()
                }
                SettingsContent::AddUser => {
                    view! { <add_user::AddUser></add_user::AddUser> }.into_view()
                }
                SettingsContent::DeleteUser => {
                    view! { <del_user::DeleteUser></del_user::DeleteUser> }.into_view()
                }
                SettingsContent::DeleteBookDetail(id) => {
                    view! {
                        <del_book::DeleteBookDetail
                            book_id=id
                            set_content=set_current_content
                        ></del_book::DeleteBookDetail>
                    }
                        .into_view()
                }
                SettingsContent::ChangePassword => {
                    view! { <change_passwd::ChangePassword></change_passwd::ChangePassword> }
                        .into_view()
                }
            }
        }}
    }
}
