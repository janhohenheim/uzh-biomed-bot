use dotenv;
use std::env;
use tbot::prelude::*;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to read .env");
    let mut bot = tbot::Bot::from_env("BOT_TOKEN").event_loop();
    bot.text(|context| async move {
        let message = format!("You sent me {}", context.text.value);
        context
            .send_message_in_reply(&message)
            .call()
            .await
            .unwrap();
    });
    bot.polling().start().await.unwrap();
}
