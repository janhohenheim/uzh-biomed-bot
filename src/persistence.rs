use crate::chat::*;
use crate::constant;
use crate::module::Module;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

pub fn read_chats() -> Result<Vec<Chat>, Box<dyn Error>> {
        let mut file = open_file(constant::SETTINGS_FILE)?;
        let mut toml = String::new();
        file.read_to_string(&mut toml)?;
        let chats: Vec<Chat> = serde_json::from_str(&toml).unwrap_or_default();

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

        let mut current_toml = String::new();
        file.read_to_string(&mut current_toml)?;
        let mut chats: Vec<Chat> = serde_json::from_str(&current_toml).unwrap_or_default();

        chats.push(chat);

        let new_toml = serde_json::to_string(&chats)?;
        file.write_all(new_toml.as_bytes())?;

        Ok(())
}

pub fn read_module(identifier: &str) -> Result<Option<Module>, Box<dyn Error>> {
        let mut file = open_file(constant::MODULES_FILE)?;
        let mut toml = String::new();
        file.read_to_string(&mut toml)?;
        let modules: Vec<Module> =
                serde_json::from_str(&toml).expect("Failed to parse modules file");

        Ok(modules
                .into_iter()
                .find(|module| module.identifier == identifier))
}
