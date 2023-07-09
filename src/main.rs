mod recipe;
mod utils;
mod web;
use tokio;

#[tokio::main]
async fn main() {
    web::web_service::select().await;
}
