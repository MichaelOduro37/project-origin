use futures_util::StreamExt;
use origin_core::daemon;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        daemon::run().await;
    });
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    let url = "ws://127.0.0.1:9944";
    println!("Connecting to {}...", url);
    match connect_async(url).await {
        Ok((ws_stream, _)) => {
            println!("Connected!");
            let (_, mut read) = ws_stream.split();
            while let Some(msg) = read.next().await {
                if let Ok(Message::Text(text)) = msg {
                    println!("Received: {}", text);
                    return;
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}
