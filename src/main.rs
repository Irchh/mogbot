mod commands;

use commands::Data;
use poise::serenity_prelude::*;
use std::env;
use dotenv::dotenv;
use log::*;

#[tokio::main]
async fn main() {
    dotenv().ok();

    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set");
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::mogping()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
    }).build();

    let intents = GatewayIntents::non_privileged();
    let client = ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    if let Err(why) = client.unwrap().start().await {
        error!("Client error: {why:?}");
    }
}
