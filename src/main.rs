use std::env;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "avatar" {
            if let Err(why) = msg
                .channel_id
                .say(&ctx.http, "https://img.atis.dev/avatar.png")
                .await
            {
                println!("Error sending message: {:?}", why);
            }
        }

        if msg.content == "LGTM" {
            if let Err(why) = msg
                .channel_id
                .say(&ctx.http, "https://img.atis.dev/lgtm.png")
                .await
            {
                println!("Error sending message: {:?}", why);
            }
        }

        if msg.content == "ぬるぽ" {
            if let Err(why) = msg
                .channel_id
                .say(&ctx.http, "ガッ")
                .await
            {
                println!("Error sending message: {:?}", why);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
