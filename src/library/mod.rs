use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;

mod endpoints;
pub mod models;

pub use endpoints::create_download_router;

use models::LibraryState;

// hardcoded for now!
const LIBRARY_PATH: &str = "/media/kobink_test";
