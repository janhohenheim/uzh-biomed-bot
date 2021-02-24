use crate::chat::*;
use crate::constant;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};

pub fn read_chats() -> Result<Vec<Chat>, Box<dyn Error>> {
        let mut file = open_file(constant::SETTINGS_FILE)?;
        let mut json = String::new();
        file.read_to_string(&mut json)?;
        let chats: Vec<Chat> = serde_json::from_str(&json).unwrap_or_default();

        Ok(chats)
}

fn open_file(file_name: &str) -> Result<File, std::io::Error> {
        OpenOptions::new()
                .create(true)
                .write(true)
                .read(true)
                .open(file_name)
}

pub fn append_chat(chat: Chat) -> Result<(), Box<dyn Error>> {
        let mut file = open_file(constant::SETTINGS_FILE)?;

        let mut current_json = String::new();
        file.read_to_string(&mut current_json)?;
        let mut chats: Vec<Chat> = serde_json::from_str(&current_json).unwrap_or_default();

        chats.push(chat);

        let new_json = serde_json::to_string(&chats)?;
        file.seek(SeekFrom::Start(0))?;
        file.set_len(0)?;
        file.write_all(new_json.as_bytes())?;

        Ok(())
}

pub fn remove_chat(chat: Chat) -> Result<Option<Chat>, Box<dyn Error>> {
        let mut file = open_file(constant::SETTINGS_FILE)?;

        let mut current_json = String::new();
        file.read_to_string(&mut current_json)?;
        let mut chats: Vec<Chat> = serde_json::from_str(&current_json).unwrap_or_default();

        let index_to_remove = chats.iter().position(|current_chat| current_chat == &chat);
        if let Some(index_to_remove) = index_to_remove {
                chats.remove(index_to_remove);
                let new_json = serde_json::to_string(&chats)?;
                file.seek(SeekFrom::Start(0))?;
                file.set_len(0)?;
                file.write_all(new_json.as_bytes())?;
                Ok(Some(chat))
        } else {
                Ok(None)
        }
}
