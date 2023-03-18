#![feature(let_chains)]

mod webhook;
mod gift;
mod config;

use std::{error::Error};
use serenity::{
  model::{channel::Message,},
  prelude::*,
  async_trait
};
use regex::RegexBuilder;
use colorful::{Colorful, Color::Red};
use tokio;
use crate::config::try_read_config;


struct Handler;

#[async_trait]
impl EventHandler for Handler {
  async fn message(&self, _: Context, msg: Message) {
    let gift_regex = RegexBuilder::new(r"(?m)(discord.gift|discord.com/gifts|discordapp.com/gifts)/(\w{16})").size_limit(1337000000).build().unwrap();
    if gift_regex.is_match(&msg.content) {
      for gift_code  in gift_regex.captures_iter(&msg.content) {
          gift::req(gift_code.get(2).map(|m| m.as_str()).unwrap().to_string(), &msg).await.unwrap();
      }
    }
  }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let intents =
      GatewayIntents::GUILD_MESSAGES
          | GatewayIntents::DIRECT_MESSAGES
          | GatewayIntents::MESSAGE_CONTENT;
  let ready = format!("{:^100}\n\r{:^100}\n\r{:^100}", "╔╗╔╦╔╦╗╦═╗╔═╗  ╔═╗╔╗╔╦╔═╗╔═╗╦═╗", "║║║║ ║ ╠╦╝║ ║  ╚═╗║║║║╠═╝║╣ ╠╦╝", "╝╚╝╩ ╩ ╩╚═╚═╝  ╚═╝╝╚╝╩╩  ╚═╝╩╚═");
  ready.rainbow();

  let token = try_read_config().map_err(|e| e.handle()).unwrap().main_token();
  let mut client = Client::builder(&token, intents)
      .event_handler(Handler)
      .await
      .expect("Couldn't make a connection to Discord on token.");
  println!("{}", format!("\n\n\n\n\n\n\n[>] Nitro Sniper just started\n").gradient(Red));
  if let Err(why) = client.start().await {
    format!("Couldn't make a connection to Discord on token : {why}. Is your token correct?").warn()
  }

  Ok(())
}