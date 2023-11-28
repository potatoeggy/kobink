mod kobo;
mod library;
mod models;
mod server;

use dotenvy::dotenv;
use server::start_server;

#[tokio::main]
async fn main() {
    dotenv().unwrap();
    tracing_subscriber::fmt().init();

    start_server().await
}
