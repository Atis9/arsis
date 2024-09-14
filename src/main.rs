use std::env;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
use log::{info, error}; // log macros

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        info!("Received message: {:?}", msg.content);

        if msg.content == "avatar" {
            info!("Processing 'avatar' command");
            if let Err(why) = msg
                .channel_id
                .say(&ctx.http, "https://img.atis.dev/avatar.png")
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if msg.content == "LGTM" {
            info!("Processing 'LGTM' command");
            if let Err(why) = msg
                .channel_id
                .say(&ctx.http, "https://img.atis.dev/lgtm.png")
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }

        if msg.content == "ぬるぽ" {
            info!("Processing 'ぬるぽ' command");
            if let Err(why) = msg
                .channel_id
                .say(&ctx.http, "ガッ")
                .await
            {
                error!("Error sending message: {:?}", why);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();

    info!("Starting the bot...");

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    info!("Token retrieved successfully");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    info!("Creating client...");
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    info!("Client created successfully");

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
