#![feature(type_alias_impl_trait)]

use uzh_biomed_bot::chat::*;
use uzh_biomed_bot::constant;
use uzh_biomed_bot::persistence::*;
use uzh_biomed_bot::scheduling::*;

use dotenv;
use std::error::Error;
use tbot::types::parameters::Text as ParseMode;
use tbot::{
    markup::*,
    prelude::*,
    types::keyboard::inline::{Button, ButtonKind},
};
type Context<T> = std::sync::Arc<tbot::contexts::Command<tbot::contexts::Text<T>>>;
type CallbackContext<T> = std::sync::Arc<tbot::contexts::DataCallback<T>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if dotenv::dotenv().is_err() {
        println!("No .env file found, reading config only from environment");
    }

    let _schedule_handle = schedule_maths();

    let mut bot = tbot::Bot::from_env("BOT_TOKEN").event_loop();
    bot.username("uzh_biomedicine_bot".to_owned());
    bot.start(handle_subscription);
    bot.command("subscribe", handle_subscription);
    bot.command("unsubscribe", handle_unsubscription);
    bot.command("links", handle_links);
    bot.data_callback(handle_callback);

    bot.polling().start().await.unwrap();
    Ok(())
}

async fn handle_subscription(context: Context<impl tbot::connectors::Connector>) {
    let chat = get_chat_from_context(&context).await;
    let chats = read_chats().expect("Failed to read chats");
    if chats
        .into_iter()
        .find(|compared_chat| compared_chat == &chat)
        .is_some()
    {
        context
                .send_message("You've already subscribed this chat to livestream announcements. You can unsubscribe again by using /unsubscribe")
                .call()
                .await
                .unwrap();
    } else {
        append_chat(chat).expect("Failed to append to chat");
        context
                .send_message("Successfully subscribed chat to livestream announcements. You can unsubscribe again by using /unsubscribe")
                .call()
                .await
                .unwrap();
    }
}

async fn get_chat_from_context(context: &Context<impl tbot::connectors::Connector>) -> Chat {
    let telegram_chat = context
        .get_chat()
        .call()
        .await
        .expect("Failed to retrieve chat");

    Chat {
        id: telegram_chat.id,
    }
}

async fn handle_unsubscription(context: Context<impl tbot::connectors::Connector>) {
    let chat = get_chat_from_context(&context).await;
    let removed_chat = remove_chat(chat).expect("Failed to read chats");

    if removed_chat.is_some() {
        context
                .send_message("You've successfully unsubscribed this chat from livestream announcements. You can subscribe again by using /subscribe")
                .call()
                .await
                .unwrap();
    } else {
        context
                .send_message("You are not subscribed to livestream announcements, so you can't unsubscribe from them. If you meant to subscribe, you can do so by using /subscribe")
                .call()
                .await
                .unwrap();
    }
}

async fn handle_links(context: Context<impl tbot::connectors::Connector>) {
    const KEYBOARD: &[&[Button]] = &[&[
        Button::new(
            "MAT 182",
            ButtonKind::CallbackData(constant::MATHS_CALLBACK),
        ),
        Button::new(
            "PHY 117",
            ButtonKind::CallbackData(constant::PHYSICS_CALLBACK),
        ),
    ]];

    context
        .send_message("Select the module you wish to see links for")
        .reply_markup(KEYBOARD)
        .call()
        .await
        .unwrap();
}

async fn handle_callback(context: CallbackContext<impl tbot::connectors::Connector>) {
    let message = match context.data.as_str() {
        constant::MATHS_CALLBACK => markdown_v2((
            "The following links are important for MAT 182:\n- ",
            link(
                "OLAT",
                "https://lms.uzh.ch/auth/RepositoryEntry/16814276984/CourseNode/85421310414617",
            ),
            "\n- ",
            link(
                "Course",
                "https://www.math.uzh.ch/index.php?id=ve_vo_det&key1=0&key2=3881&semId=41",
            ),
            "\n- ",
            link(
                "Exercises",
                "https://w3.math.uzh.ch/my/index.php?id=lecture",
            ),
        ))
        .to_string(),
        constant::PHYSICS_CALLBACK => markdown_v2((
            "The following links are important for PHY 117:\n- ",
            link(
                "OLAT",
                "https://lms.uzh.ch/auth/RepositoryEntry/16830890450/CourseNode/85421310414617",
            ),
            "\n- ",
            link("Course", "https://www.physik.uzh.ch/de/lehre/PHY117/HS2020"),
        ))
        .to_string(),
        _ => panic!("Invalid callback"),
    };

    let chat_id = if let tbot::types::callback::query::Origin::Message(message) = &context.origin {
        message.chat.id
    } else {
        return;
    };

    let call_result = context
        .bot
        .send_message(chat_id, ParseMode::markdown_v2(&message))
        .call()
        .await;

    if let Err(err) = call_result {
        dbg!(err);
    }
}
