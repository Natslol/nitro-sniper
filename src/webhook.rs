use std::error::Error;
use std::time::Instant;
use colorful::Colorful;
use reqwest::{Client, Body};
use serenity::model::channel::Message;
use crate::config::{try_read_config};

#[derive(serde::Deserialize, serde::Serialize, )]
pub struct Webhook {
    username: &'static str,
    avatar_url: &'static str,
    embeds: Vec<Embed>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Embed {
    title: &'static str,
    description: String,
    color: i64,
    fields: Option<Vec<Field>>,
    timestamp: String,
    footer: Footer
}

#[derive(serde::Deserialize, serde:: Serialize)]
pub struct Field {
    name: &'static str,
    value: String,
    inline: Option<bool>
}

#[derive(serde::Deserialize, serde:: Serialize)]
pub struct Footer {
    text: &'static str,
    icon_url: Option<String>
}

impl Webhook {
    pub fn content(gift_code: String, claimed_in: String, msg: &Message) -> Self {
        Webhook {
            username: "Nitro Sniper 🛐",
            avatar_url: "https://i.pinimg.com/564x/c5/2a/1c/c52a1c50324930746f4d9c855d9bb07e.jpg",
            embeds: vec![
                Embed {
                    title: "🐍 - Mamba Nitro Sniper",
                    description: format!("Claimed Nitro !\nhttps://discord.gift/{}", gift_code),
                    color: 65280,
                    fields: Some(vec![
                        Field {
                            name: "Claimed in: ",
                            value: format!("{} seconds", claimed_in),
                            inline: Some(true)
                        },
                        Field {
                            name: "Nitro sent by: ",
                            value: format!("{}#{:0>4}", msg.author.name, msg.author.discriminator),
                            inline: Some(false)
                        },
                        Field {
                            name: "Message: ",
                            value: format!("[Here!](https://discordapp.com/channels/{}/{}/{})", msg.guild_id.map_or_else(|| "@me".to_string(), |g| g.to_string()), msg.channel_id.to_string(), msg.id.to_string()),
                            inline: Some(true)
                        },
                        Field {
                            name: "Channel: ",
                            value: format!("<#{}>", msg.channel_id),
                            inline: Some(false)
                        }]),
                    footer: Footer {
                        text: "Mamba Nitro Sniper",
                        icon_url: Some(msg.author.avatar_url().clone().unwrap())
                    },
                    timestamp: chrono::Local::now().to_rfc3339()
                }
            ]
        }
    }
}

pub async fn send(gift_code: String, now: Instant, msg: &Message) -> Result<(), Box<dyn Error>> {
    let webhook = Webhook::content(gift_code.clone(), format!("{}.{:0>3}", now.elapsed().as_secs(), now.elapsed().as_millis()), msg);
    let req = Client::new()
        .post(try_read_config().map_err(|e| e.handle()).unwrap().webhook().unwrap())
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string_pretty(&webhook).unwrap()))
        .send()
        .await?;
    match req.status().as_u16() {
        204 => {
            println!("{}", format!("[{}] Sent to webhook.\n", chrono::Local::now().format("%H:%M")).green());
        },
        400 => {
            println!("{}", format!("[{}] Bad request webhook (did you change something ?).\n", chrono::Local::now().format("%H:%M")).yellow());
        },
        404 => {
            println!("{}", format!("[{}] Your webhook doesn't exist.\n", chrono::Local::now().format("%H:%M")).light_red());
        },
        405 => {
            println!("{}", format!("[{}] There was an error on Discord's side.\n", chrono::Local::now().format("%H:%M")).red())
        },
        429 => {
            println!("{}", format!("[{}] Webhook Rate-limited...\n", chrono::Local::now().format("%H:%M")).red())
        }
        _ => {
            println!("{}", format!("[{}] Received unknown response.\n", chrono::Local::now().format("%H:%M")).red())
        }
    }
    Ok(())
}
