mod server;

use server::start_server;

#[tokio::main]
async fn main() {
    return start_server().await;
}
