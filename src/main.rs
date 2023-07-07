mod server;

use server::start_server;
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    return start_server().await;
}
