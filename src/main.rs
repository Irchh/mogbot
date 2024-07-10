mod commands;

use std::env;
use dotenv::dotenv;
use log::*;
use serenity::{all::{Ready, standard::macros::hook, Command, Context, CreateInteractionResponse, CreateInteractionResponseMessage, EventHandler, GatewayIntents, Interaction, Message, StandardFramework}, async_trait, Client};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            debug!("Recieved command: {:?}", command);
            let content = match command.data.name.as_str() {
                "ping" => Some(commands::ping::run(&command.data.options())),
                _ => Some("not implemented".to_string()),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why:?}");
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let _= Command::create_global_command(&ctx.http, commands::ping::register()).await;
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set");
    let mut client = Client::builder(&token, GatewayIntents::empty()).event_handler(Handler).await.expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
    println!("Hello, world!");
}
