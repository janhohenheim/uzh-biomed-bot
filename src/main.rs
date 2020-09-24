use uzh_biomed_bot::chat::*;
use uzh_biomed_bot::file::*;

use async_std::task;
use clokwerk::Interval::*;
use clokwerk::{Scheduler, TimeUnits};
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
    bot.start(|context| {
        async move {
            let telegram_chat = context
                .get_chat()
                .call()
                .await
                .expect("Failed to retrieve chat");

            let chat = Chat {
                id: telegram_chat.id,
            };
            append_chat(chat).expect("Failed to append to chat");
        }
    });
    bot.text(|context| {
        async move {
            let message = format!("You sent me {}", context.text.value);
            context
                .send_message_in_reply(&message)
                .call()
                .await
                .expect("Failed to retrieve message");
        }
    });
    bot.polling().start().await.unwrap();
    Ok(())
}

fn schedule_maths() {
    let mut scheduler = Scheduler::with_tz(chrono::Utc);
    let mut math_links = get_math_links();
    scheduler
        .every(Tuesday)
        .at("14:20:17")
        .and_every(Thursday)
        .at("15:00")
        .run(move || {
            let current_link = math_links.pop().unwrap_or("Whoops, there is no link");
            task::block_on(broadcast_message("Foo"));
        });
    let thread_handle = scheduler.watch_thread(Duration::from_millis(100));
}

async fn broadcast_message(message: &'static str) {
    const CHAT: chat::Id = chat::Id(0);
    const MESSAGE: &str = "`tbot` is a super-cool crate!";

    let bot = tbot::Bot::from_env("BOT_TOKEN");
    bot.send_message(CHAT, message).call().await.unwrap();
}

fn get_math_links() -> Vec<&'static str> {
    vec!["foo"]
}
