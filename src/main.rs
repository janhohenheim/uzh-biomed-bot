use uzh_biomed_bot::chat::*;
use uzh_biomed_bot::file::*;

use dotenv;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::thread;
use std::time::Duration;
use tbot::prelude::*;
use tbot::types::{chat, parameters::Text};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().expect("Failed to load .env");

    // schedule_maths();

    let mut bot = tbot::Bot::from_env("BOT_TOKEN").event_loop();
    bot.start(|context| async move {
        let telegram_chat = context
            .get_chat()
            .call()
            .await
            .expect("Failed to retrieve chat");

        let chat = Chat {
            id: telegram_chat.id,
        };
        append_chat(chat).expect("Failed to append to chat");
    });
    bot.text(|context| async move {
        let message = format!("You sent me {}", context.text.value);
        context
            .send_message_in_reply(&message)
            .call()
            .await
            .expect("Failed to retrieve message");
    });
    bot.polling().start().await.unwrap();
    Ok(())
}
