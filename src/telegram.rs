use crate::persistence::*;
use tbot::markup::*;

use std::error::Error;

pub struct LiveStreamViewModel {
    pub identifier: String,
    pub name: String,
    pub link: Option<String>,
}

pub async fn broadcast_live_stream(view_model: LiveStreamViewModel) -> Result<(), Box<dyn Error>> {
    const INTRO: &'static str = "📢 A livestream starting in 15 Minutes: \n";
    let link_name = format! {"{} {}", view_model.identifier, view_model.name};
    let message = if let Some(live_stream_link) = view_model.link {
        let link_name = format! {"{} {}", view_model.identifier, view_model.name};
        markdown_v2((INTRO, link(link_name, live_stream_link))).to_string()
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
        bot.send_message(chat.id, &message).call().await?;
    }
    Ok(())
}
