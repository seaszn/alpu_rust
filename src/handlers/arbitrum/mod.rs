mod types;
mod data_feed;

pub async fn init() {
    _ = data_feed::init().await
}