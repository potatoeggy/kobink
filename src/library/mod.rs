use std::sync::Mutex;

use lazy_static::lazy_static;

pub mod models;

use models::LibraryState;

// hardcoded for now!
const LIBRARY_PATH: &str = "/media/kobink_test";

// temporary global state - we should use a db
lazy_static! {
    pub static ref LIBRARY: Mutex<LibraryState> = Mutex::new(LibraryState::new(LIBRARY_PATH));
}
