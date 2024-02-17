use crate::{errors::AudioAppError, server_api::User};
use crate::server_api::auth::*;
use crate::ui::main_page::MainPage;

use leptos::*;
use leptos_meta::{provide_meta_context, Link, Stylesheet};
use leptos_router::*;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Todo {
    id: u32,
    user: Option<User>,
    title: String,
    created_at: String,
    completed: bool,
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Link rel="shortcut icon" href="/favicon.ico"/>
        <Link rel="manifest" href="/manifest.json"/>
        <Stylesheet id="leptos" href="/pkg/audiobookroom.css"/>
        <div class="h-screen">
            <Router>
                <Routes>
                    <Route path="/" view=Main/>
                </Routes>

            </Router>
        </div>
    }
}

#[component]
fn Main() -> impl IntoView {
    let logout: Action<Logout, Result<(), ServerFnError>> = create_server_action::<Logout>();
    let login = create_server_action::<Login>();
    let signup = create_server_action::<Signup>();
    let user = create_resource(
        move || {
            (
                logout.version().get(),
                login.version().get(),
                signup.version().get(),
            )
        },
        move |_| get_user(),
    );
    view! {
        <div class="h-full bg-gray-300">
            <Transition fallback=move || {
                view! { <span>"Loading..."</span> }
            }>
                {move || {
                    user.get()
                        .map(|user| {
                            user.map(|user| {
                                    match user {
                                        Some(user) => {
                                            view! { <MainPage user=user logout=logout/> }.into_view()
                                        }
                                        None => {
                                            view! { <LoginPage login=login signup=signup/> }.into_view()
                                        }
                                    }
                                })
                                .map_err(|_e| { AudioAppError::InternalServerError })
                        })
                }}

            </Transition>
        </div>
    }
}
#[component]
fn LoginPage(
    login: Action<Login, Result<(), ServerFnError>>,
    signup: super::ServerAction<Signup, ()>,
) -> impl IntoView {
    let username: NodeRef<html::Input> = create_node_ref();
    let password: NodeRef<html::Input> = create_node_ref();
    let remember: NodeRef<html::Input> = create_node_ref();

    let on_login = move |_| {
        let username = username().unwrap().value();
        let password = password().unwrap().value();
        let remember = remember().unwrap().checked();
        let remember = if remember {
            Some("Remember".to_string())
        } else {
            None
        };
        login.dispatch(Login {
            username,
            password,
            remember,
        });
    };
    let on_signup = move |_| {
        let username = username().unwrap().value();
        let password = password().unwrap().value();
        let remember = remember().unwrap().checked();
        let remember = if remember {
            Some("Remember".to_string())
        } else {
            None
        };
        signup.dispatch(Signup {
            username,
            password,
            remember,
        });
    };
    let have_user = create_resource(
        || {},
        move |_| async move {
            use crate::server_api::auth::have_user;
            have_user().await
        },
    );

    view! {
        <div class="flex min-h-full flex-col justify-center px-6 py-12 lg:px-8">
            // the top logo
            <div class="sm:mx-auto sm:w-full sm:max-w-sm">
                <img class="mx-auto h-10 w-auto" src="/icon.png" alt="Your Company"/>
                <h2 class="mt-10 text-center text-2xl font-bold leading-9 tracking-tight text-gray-900">
                    <Transition fallback=move || {}>
                        {move || {
                            have_user
                                .get()
                                .map(|have_user| {
                                    match have_user {
                                        Ok(have_user) => {
                                            if have_user {
                                                view! { <span>Login your account</span> }
                                            } else {
                                                view! { <span>Create a new account</span> }
                                            }
                                        }
                                        Err(_) => {
                                            view! { <span>"Error"</span> }
                                        }
                                    }
                                })
                        }}

                    </Transition>
                </h2>
            </div>

            <div class="mt-10 sm:mx-auto sm:w-full sm:max-w-sm">
                <form class="space-y-6">
                    <div>
                        <label
                            for="username"
                            class="block text-sm font-medium leading-6 text-gray-900"
                        >
                            User Name
                        </label>
                        <div class="mt-2">
                            <input
                                id="username"
                                name="username"
                                type="text"
                                required
                                ref=username
                                class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                            />
                        </div>
                    </div>

                    <div>
                        <label
                            for="password"
                            class="block text-sm font-medium leading-6 text-gray-900"
                        >
                            Password
                        </label>

                        <div class="mt-2">
                            <input
                                id="password"
                                name="password"
                                type="password"
                                ref=password
                                autocomplete="current-password"
                                required
                                class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                            />
                        </div>
                    </div>
                    <div class="flex justify-between items-center ">
                        <p>"Remember me? "</p>
                        <input ref=remember type="checkbox" name="remember" class="auth-input"/>
                    </div>
                    <div class="flex space-x-2">
                        <Transition fallback=move || {}>
                            {move || {
                                have_user
                                    .get()
                                    .map(|have_user| {
                                        match have_user {
                                            Ok(have_user) => {
                                                if have_user {
                                                    view! {
                                                        <button
                                                            type="button"
                                                            on:click=on_login
                                                            class="flex w-full justify-center rounded-md bg-indigo-600 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
                                                        >
                                                            Sign in
                                                        </button>
                                                    }
                                                        .into_view()
                                                } else {
                                                    view! {
                                                        <button
                                                            type="button"
                                                            on:click=on_signup
                                                            class="flex w-full justify-center rounded-md bg-indigo-600 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
                                                        >
                                                            Sign up
                                                        </button>
                                                    }
                                                        .into_view()
                                                }
                                            }
                                            Err(_) => view! { <span>"Error"</span> }.into_view(),
                                        }
                                    })
                            }}

                        </Transition>

                    </div>

                </form>
            </div>
        </div>
    }
}
