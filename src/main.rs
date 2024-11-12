use log::{error, info};
use poise::serenity_prelude as serenity;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let response = "pong";
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn lgtm(ctx: Context<'_>) -> Result<(), Error> {
    let response = "https://img.atis.dev/lgtm";
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn lgtm_to(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("<@{}> https://img.atis.dev/lgtm", u.id);
    ctx.say(response).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("Starting the bot...");

    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    info!("Token retrieved successfully");

    // let intents = serenity::GatewayIntents::non_privileged();
    let intents = serenity::GatewayIntents::GUILD_MESSAGES
        | serenity::GatewayIntents::DIRECT_MESSAGES
        | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![ping(), lgtm(), lgtm_to()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    info!("Creating client...");
    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    info!("Client created successfully");
    client.unwrap().start().await.unwrap();
}
