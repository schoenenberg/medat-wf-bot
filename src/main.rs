use telegram_bot::{Error, UpdateKind};
use crate::client::Client;
use futures::StreamExt;

mod client;
mod word_generator;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut client = Client::new().await;

    // Fetch new updates via long poll method
    let mut stream = client.api.stream();
    while let Some(update) = stream.next().await {
        // If the received update contains a new message...
        if let Ok(update) = update {
            if let UpdateKind::Message(message) = update.kind {
                client.process_msg(message).await?;
            }
        }
    }
    Ok(())
}
