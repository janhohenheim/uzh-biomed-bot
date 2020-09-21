use dotenv;
use std::env;
use tbot::prelude::*;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to read .env");
    let mut bot = tbot::Bot::from_env("BOT_TOKEN").event_loop();
}
