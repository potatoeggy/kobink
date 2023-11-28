use std::sync::Mutex;

use lazy_static::lazy_static;

mod endpoints;
pub mod models;

pub use endpoints::create_download_router;

use models::LibraryState;

// hardcoded for now!
const LIBRARY_PATH: &str = "/media/kobink_test";

// temporary global state - we should use a db
lazy_static! {
    pub static ref LIBRARY: Mutex<LibraryState> = Mutex::new(LibraryState::new(LIBRARY_PATH));
}

// TODO: OR we could even use the builtin axum state manager
