use crate::chat::*;
use crate::constant;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

pub fn append_chat(chat: Chat) -> Result<(), Box<dyn Error>> {
    let mut file = open_settings_file()?;

    let mut current_toml = String::new();
    file.read_to_string(&mut current_toml)?;
    let mut chats: Vec<Chat> = toml::from_str(&current_toml).unwrap_or_default();

    chats.push(chat);

    let new_toml = toml::to_string(&chats)?;
    file.write_all(new_toml.as_bytes())?;

    Ok(())
}

fn open_settings_file() -> Result<File, std::io::Error> {
    OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open(constant::SETTINGS_FILE)
}
