use my_rest_api::api::start_server;

#[tokio::main]
async fn main() {
    start_server().await
}
