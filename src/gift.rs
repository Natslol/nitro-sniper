use std::{
    error::Error,
    time::{
        Instant
    }
};
use colorful::Colorful;
use reqwest;
use serenity::model::channel::Message;
use crate::webhook;
use crate::config::try_read_config;


pub async fn req(gift_code: String, msg: &Message) -> Result<(), Box<dyn Error>> {
    let now = Instant::now();
    let client =  reqwest::Client::new();
    let req = client.post(format!("https://discord.com/api/v8/entitlements/gift-codes/{}/redeem", gift_code))
        .header("Authorization", try_read_config().map_err(|e| e.handle()).unwrap().main_token())
        .header("Content-Length", 0)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36")
        .send()
        .await?;
    match req.status().as_u16() {
        200 => {
            success(gift_code, now, msg).await?;
        },
        400 => {
            println!("{}", format!("[{}] Code was already redeemed.", chrono::Local::now().format("%H:%M")).yellow());
        },
        404 => {
            println!("{}", format!("[{}] Code was fake or expired.", chrono::Local::now().format("%H:%M")).light_red());
        },

        405 => {
            println!("{}", format!("[{}] There was an error on Discord's side.", chrono::Local::now().format("%H:%M")).red())
        },
        429 => {
            println!("{}", format!("[{}] Rate-limited...", chrono::Local::now().format("%H:%M")).red())
        }
        _ => {
            println!("{}", format!("[{}] Received unknown response.", chrono::Local::now().format("%H:%M")).red())
        }
    }
    Ok(())
}

async fn success(gift_code: String, now: Instant, msg: &Message) -> Result<(), Box<dyn Error>> {
    println!("{}", "GG! You claimed Nitro!".green());
    if try_read_config().map_err(|e| e.handle()).unwrap().webhook() != Some("".to_string()) {
        webhook::send(gift_code, now, msg).await?;
    }
    Ok(())
}
