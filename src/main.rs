mod commands;
mod functions;

use std::env;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use serenity::async_trait;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

struct Handler {
    is_loop_running: AtomicBool,
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {

            let guild_id = GuildId(
                env::var("GUILD_ID")
                    .expect("Expected GUILD_ID in environment")
                    .parse()
                    .expect("GUILD_ID must be an integer"),
            );

            let content = match command.data.name.as_str() {
                "ping" => commands::ping::run(&command.data.options),
                "debug" => commands::debug::run(&ctx, guild_id),
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        // GUILD COMMANDS
        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::ping::register(command))
                .create_application_command(|command| commands::wonderful_command::register(command))
                .create_application_command(|command| commands::debug::register(command))
        })
        .await;

        println!("I now have the following guild slash commands: {:#?}", commands);
    }

    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        println!("Cache built successfully!");

        let ctx = Arc::new(ctx);

        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        if !self.is_loop_running.load(Ordering::Relaxed) {

            let ctx1 = Arc::clone(&ctx);
            tokio::spawn(async move {
                loop {
                    functions::update_users::members(&ctx1, guild_id).await;
                    tokio::time::sleep(Duration::from_secs(3)).await;
                    functions::update_users::members_online(&ctx1, guild_id).await;
                    println!("Member count updated. Next update in 601 Seconds");
                    tokio::time::sleep(Duration::from_secs(601)).await;
                }
            });

            // Vorlage für 2nd Timer
            /* let ctx2 = Arc::clone(&ctx);
            tokio::spawn(async move {
                loop {
                    //code
                    tokio::time::sleep(Duration::from_secs(60)).await;
                }
            }); */

            self.is_loop_running.swap(true, Ordering::Relaxed);
        }
    }
}

#[tokio::main]
async fn main() {
    let _cid = env::var("CHANNELID_VOICE").expect("channelid_voice").parse::<u64>().expect("channelid_voice: u64");
    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::GUILD_MEMBERS;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler {
            is_loop_running: AtomicBool::new(false),
        })
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
