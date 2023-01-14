use server::datastructure::Tnet;
use tokio;

#[tokio::main]
async fn main() {
    Tnet::run().await;
}