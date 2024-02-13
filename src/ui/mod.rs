// use leptos::{create_memo, Memo, ReadSignal, ServerFnError, SignalGet};
use leptos::ServerFnError;

pub mod app;
pub mod main_authors;
pub mod main_books;
pub mod main_index;
pub mod main_page;
pub mod main_setting;
pub mod player;
pub type ServerAction<T, I> = leptos::Action<T, Result<I, ServerFnError>>;
// pub fn translate_time(time: i64) -> (u32, u32) {
//     let min = time / 60;
//     let sec = time % 60;
//     (min as u32, sec as u32)
// }
// pub fn create_time_memo(time: ReadSignal<f64>) -> Memo<(u32, u32)> {
//     create_memo(move |_| {
//         let time = time.get();
//         translate_time(time as i64)
//     })
// }
