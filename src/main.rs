use std::env;

use futures::StreamExt;
use telegram_bot::*;
mod commands;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let api = Api::new(token);

    // Fetch new updates via long poll method
    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        // If the received update contains a new message...
        let update = update?;
        if let UpdateKind::Message(message) = update.kind {
            if let MessageKind::Text { ref data, .. } = message.kind {
                match data.as_str() {
                    "/start" => commands::start(&api, &message).await?,
                    command if command.starts_with("/code") => {
                        commands::code(&api, &message, data).await?
                    }
                    _ => commands::invalid(&api, &message).await?,
                }

                // Print received text message to stdout.
                println!("<{}>: {}", &message.from.first_name, data);

                ()
            }
        }
    }
    Ok(())
}
