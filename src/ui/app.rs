use crate::server_api::auth::*;
use crate::server_api::User;
use crate::ui::login_page::LoginPage;
use crate::ui::main_authors::AuthorDetailPage;
use crate::ui::main_authors::AuthorIndex;
use crate::ui::main_books::BookDetail;
use crate::ui::main_books::BookIndex;
use crate::ui::main_books::ChapterView;
use crate::ui::main_index::MainIndex;
use crate::ui::main_page::MainPage;
use crate::ui::main_setting::MainSettings;

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
#[derive(Clone, Copy)]
pub struct LogoutContext(pub Action<Logout, Result<(), ServerFnError>>);

#[component]
pub fn App() -> impl IntoView {
    // setup the actions
    let logout: Action<Logout, Result<(), ServerFnError>> = create_server_action::<Logout>();
    let login = create_server_action::<Login>();
    let signup = create_server_action::<Signup>();
    let user = create_resource(
        move || {
            (
                login.version().get(),
                signup.version().get(),
                logout.version().get(),
            )
        },
        move |_| async move { get_user().await },
    );
    // setup context
    provide_meta_context();
    provide_context(LogoutContext(logout));

    view! {
        <Link rel="shortcut icon" href="/favicon.ico"/>
        <Link rel="manifest" href="/manifest.json"/>
        <Stylesheet id="leptos" href="/pkg/audiobookroom.css"/>
        <div class="h-screen">
            <Router>
                <Routes>
                    <Route
                        path="/login"
                        view=move || {
                            view! {
                                <Transition fallback=move || {
                                    view! { <p>"loading"</p> }
                                }>

                                    {move || {
                                        user.get()
                                            .map(|user| {
                                                match user {
                                                    Ok(user) => {
                                                        match user {
                                                            Some(_user) => {
                                                                view! {
                                                                    <p>"already login"</p>
                                                                    <a class="text-blue-500" href="/">
                                                                        {"go to main"}
                                                                    </a>
                                                                }
                                                                    .into_view()
                                                            }
                                                            None => {
                                                                view! { <LoginPage login=login signup=signup/> }.into_view()
                                                            }
                                                        }
                                                    }
                                                    Err(_err) => view! { <p>{"error"}</p> }.into_view(),
                                                }
                                            })
                                    }}

                                </Transition>
                            }
                        }
                    />

                    <Route
                        path="/"
                        view=move || {
                            view! {
                                <Transition fallback=move || {
                                    view! { <p>"loading"</p> }
                                }>

                                    {move || {
                                        user.get()
                                            .map(|user| {
                                                match user {
                                                    Ok(user) => {
                                                        match user {
                                                            Some(user) => view! { <MainPage user=user/> }.into_view(),
                                                            None => {
                                                                view! {
                                                                    <p>"login required"</p>
                                                                    <a class="text-blue-500" href="/login">
                                                                        {"go to login"}
                                                                    </a>
                                                                }
                                                                    .into_view()
                                                            }
                                                        }
                                                    }
                                                    Err(_err) => view! { <p>{"error"}</p> }.into_view(),
                                                }
                                            })
                                    }}

                                </Transition>
                            }
                        }
                    >

                        <Route
                            path=""
                            view=move || {
                                view! { <MainIndex/> }
                            }
                        />

                        <Route path="books" view=MountSingle>
                            <Route path="" view=BookIndex/>
                            <Route path="book/:book_id" view=BookDetail/>
                            <Route path="book/:book_id/chapter/:chapter_id" view=ChapterView/>
                        </Route>

                        <Route path="authors" view=MountSingle>
                            <Route path="" view=AuthorIndex/>
                            <Route path="auhtor/:author_id" view=AuthorDetailPage/>
                        </Route>

                        <Route path="settings" view=MainSettings/>

                    </Route>

                </Routes>

            </Router>
        </div>
    }
}

#[component]
pub fn MountSingle() -> impl IntoView {
    view! { <Outlet/> }
}
