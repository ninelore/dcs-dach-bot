mod commands;

use std::env;
use serenity::async_trait;
use serenity::model::prelude::{Message, Ready};
use serenity::prelude::*;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::StandardFramework;

use crate::commands::ping::*;

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("?"))
        .group(&GENERAL_GROUP);

    let token = env::var("DISCORD_TOKEN").expect("token");

    let intents = GatewayIntents::non_privileged() 
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
