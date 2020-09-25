use crate::chat::*;
use crate::constant;
use crate::module::Module;
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
        file.write_all(new_json.as_bytes())?;

        Ok(())
}

pub fn read_module(identifier: &str) -> Result<Option<Module>, Box<dyn Error>> {
        let mut file = open_file(constant::MODULES_FILE)?;
        let mut json = String::new();
        file.read_to_string(&mut json)?;
        let modules: Vec<Module> =
                serde_json::from_str(&json).expect("Failed to parse modules file");

        Ok(modules
                .into_iter()
                .find(|module| module.identifier == identifier))
}
