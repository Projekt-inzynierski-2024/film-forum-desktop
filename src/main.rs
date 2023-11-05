use client::ApiClient;

mod client;

#[tokio::main]
async fn main() {
    let _client = ApiClient::new();
}
