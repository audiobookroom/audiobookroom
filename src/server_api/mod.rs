use serde::{Deserialize, Serialize};

pub mod auth;
pub mod book;
pub mod progress;

pub mod authors;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PageItems<T> {
    pub page: u64,
    pub max_item: u64,
    pub number_of_items: u64,
    pub number_of_pages: u64,
    pub items: Vec<T>,
}
