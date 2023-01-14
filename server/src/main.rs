use server::datastructure::Tnet;
use tokio;

#[tokio::main]
async fn main() {
    let mut tnet = Tnet::new();
    tnet.run().await;
}