# Nitro Sniper
> **Warning** use this tool for educational purposes only, I'm not responsable for any discord ban

## Please be sure to have a phone number on your account or you'll be phone locked


## Video

> W.I.P. -> [Channel](https://www.youtube.com/channel/UC7KMFrYHQzmXCbomARoP9zg)

## Installation

#### Option 1:
1. Download [here](https://github.com/Natslol/nitro-sniper/releases/)
2. Run the executable once
3. [Edit](https://github.com/Natslol/nitro-sniper#Configuration) `config.json` that has just been created
4. Run the executable and it'll works !

#### Option 2:
1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Git clone the repo: `git clone https://github.com/Natslol/nitro-sniper.git`
3. Open CMD in the `nitro-sniper` directory that has just been cloned
4. And build the project `cargo build --release`
5. Go to `./target/release`
6. Run the executable once
7. [Edit](https://github.com/Natslol/nitro-sniper#Configuration) `config.json` that has just been created
8. Run the executable and it'll works !

## Configuration

you have to config `config.json` like this:

```json
{
  "main_token": "YOUR_TOKEN", 
  "webhook": "YOUR_WEBHOOK"
}
```

#### Exemple: 

```json
{
  "main_token": "OTczOTMxNjA2NDQyMTM5Njg4.IloveN.ats0QYxSG_4LRG19Mt5laVMY_uGG8rbdxxOyr4",
  "webhook": "https://canary.discord.com/api/webhooks/1337133713371337133/NatsIsVeryCoolAndNiceButYouHaveToPutAWebhookHereOrItWillSendNothing!"
}
```


