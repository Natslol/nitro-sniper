use std::{fs::File, io::{Read, Write}, io, process};
use regex::RegexBuilder;
use serde_json;
use colorful::{Colorful};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Config {
    main_token: String,
    webhook: String
}

impl Default for Config {
    fn default() -> Self {
        Config {
            main_token: "YOUR_TOKEN".to_string(),
            webhook: "".to_string()
        }
    }
}


impl Config {
    pub fn main_token(&self) -> String {
        match RegexBuilder::new(r"(?m)[\w-]{24}\.[\w-]{6}\.[\w-]{25,110}").size_limit(1337000000).build().unwrap().is_match(self.main_token.trim()) {
            true => self.main_token.clone().to_string(),
            false => { format!("Invalid token (doesn't match the regex)").warn(); io::stdin().read_line(&mut String::new()).unwrap(); process::exit(1337)}
        }
    }
    pub fn webhook(&self) -> Option<String> {
        if self.webhook != "" {
            match RegexBuilder::new(r"(?m)https://(canary.|ptb.)?(discord.com|discordapp.com)/api/webhooks/\d+/\S+").size_limit(1337000000).build().unwrap().is_match(self.webhook.trim()) {
                true => Some(self.webhook.clone()),
                false => { format!("Invalid webhook (doesn't match the regex)").warn(); io::stdin().read_line(&mut String::new()).unwrap(); process::exit(1337) }
            }
        } else {
            None
        }
    }
}

pub enum ConfigReadError {
    NoSuchFile,
    FailedReading,
    MalformedString(String)
}

impl ConfigReadError {
    pub fn handle(&self) {
        match self {
            ConfigReadError::NoSuchFile => match config_create() {
                Ok(_) => {
                    "No previous config file found, configure config.json file I just created!".warn();
                    io::stdin().read_line(&mut String::new()).unwrap();
                }
                Err(_) => {
                    "No previous config file found, configure config.json file I just created!".warn();
                    io::stdin().read_line(&mut String::new()).unwrap();
                }
            },
            ConfigReadError::FailedReading => {
                "I wasn't able to read/open your config.json.".warn();
                io::stdin().read_line(&mut String::new()).unwrap();
            },
            ConfigReadError::MalformedString(reason) => {
                format!("I couldn't read your config. Did you format correctly ?\n-> {}", reason).warn();
                io::stdin().read_line(&mut String::new()).unwrap();
            }
        }
    }
}

pub enum ConfigWriteError {
    FailedCreating,
    FailedWriting
}

pub fn try_read_config() -> Result<Config, ConfigReadError> {
    let mut file = File::open("./config.json").map_err(|_| ConfigReadError::NoSuchFile)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|_| ConfigReadError::FailedReading)?;
    serde_json::from_str::<Config>(&contents).map_err(|e| ConfigReadError::MalformedString(e.to_string()))
}

pub fn config_create() -> Result<(), ConfigWriteError> {
    let config = Config::default();
    let mut file = File::create("config.json").map_err(|_| ConfigWriteError::FailedCreating)?;
    file.write_all(serde_json::to_string_pretty(&config).unwrap().as_bytes(),).map_err(|_| ConfigWriteError::FailedWriting)?;
    Ok(())
}