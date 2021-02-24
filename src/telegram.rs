use crate::persistence::*;
use std::error::Error;
use tbot::markup::*;
use tbot::types::chat::Kind;
use tbot::types::parameters::Text as ParseMode;

pub struct LiveStreamViewModel {
    pub identifier: String,
    pub name: String,
    pub link: Option<String>,
    pub password: Option<String>,
}

pub async fn broadcast_live_stream(view_model: LiveStreamViewModel) -> Result<(), Box<dyn Error>> {
    const INTRO: &'static str = "📢 A livestream is starting in 15 Minutes: \n";
    let link_name = format! {"{} {}", view_model.identifier, view_model.name};
    let message = if let Some(live_stream_link) = view_model.link {
        let link_name = format! {"{} {}", view_model.identifier, view_model.name};
        let password = if let Some(password) = view_model.password {
            format!("\nPassword: {}", password)
        } else { 
            "".to_owned()
        };
        markdown_v2((INTRO, link(link_name, live_stream_link), password)).to_string()
    } else {
        markdown_v2((
            INTRO,
            link_name,
            " ⚠️ Whoops, there should be a link here, but there is none. Contact @jnferner ⚠️",
        ))
        .to_string()
    };
    broadcast_message(message).await
}

async fn broadcast_message(message: String) -> Result<(), Box<dyn Error>> {
    let chats = read_chats()?;

    let bot = tbot::Bot::from_env("BOT_TOKEN");
    for chat in chats {
        let message = bot
            .send_message(chat.id, ParseMode::markdown_v2(&message))
            .call()
            .await?;
        match message.chat.kind {
            Kind::Private { .. } => {}
            _ => bot.pin_chat_message(chat.id, message.id).call().await?,
        };
    }
    Ok(())
}
