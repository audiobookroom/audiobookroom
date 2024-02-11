use leptos::ServerFnError;

pub mod app;
pub mod main_page;
pub mod player;
pub mod main_index;
pub mod main_authors;
pub mod main_books;
pub mod main_setting;
pub type ServerAction<T, I> = leptos::Action<T, Result<I, ServerFnError>>;
