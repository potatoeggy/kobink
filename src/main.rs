mod server;

use server::start_server;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    return start_server().await;
}
