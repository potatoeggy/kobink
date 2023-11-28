pub mod endpoints;
pub mod library;
pub mod models;
pub mod schema;

pub use endpoints::create_kobo_router;

const KOBO_STOREAPI_URL: &'static str = "https://storeapi.kobo.com";
const KOBO_IMAGEHOST_URL: &'static str = "https://kbiages1-a-akamaihd.net";
const SYNC_ITEM_LIMIT: u32 = 100;

pub fn get_store_url_for_current_request() {
    unimplemented!()
}
