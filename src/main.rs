mod commands;
mod functions;
mod util;

use std::env;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;


use dotenv;

use serenity::gateway::ActivityData;
use serenity::prelude::{EventHandler, Context, GatewayIntents};
use serenity::{async_trait, Client};
use serenity::all::{Interaction, GuildId, Ready, Message};


struct Handler {
  is_loop_running: AtomicBool,
}

#[async_trait]
impl EventHandler for Handler {
  async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
    match &interaction {
      Interaction::Command(command) => {
        match command.data.name.as_str() {
          "debug" => commands::debug::run(&ctx, &command).await,
          "role" => commands::rolepicker::create_picker(&ctx, &command).await,
          _ => ()
        };
      }

      Interaction::Component(component) => {
        match component.data.custom_id.as_str() {
          "rolepicker" => commands::rolepicker::interaction(&ctx, &component).await,
          "bearbeiten" | "takeover" | "freigeben" | "close" => functions::modmsg::interaction(&ctx, &component).await,
          _ => (),
        }
      }
      _ => println!("INFO: Unhandled Interaction caught")
    }
  }

  async fn message(&self, ctx: Context, msg: Message) {
    if msg.is_private() && !msg.is_own(&ctx) {
      functions::modmsg::alert_moderators(&ctx, msg).await;
    }
  }

  async fn ready(&self, ctx: Context, ready: Ready) {
    println!("{} is connected!", ready.user.name);

    let guild_id = GuildId::new(
      env::var("GUILD_ID")
        .expect("Expected GUILD_ID in environment")
        .parse()
        .expect("GUILD_ID must be an integer"),
    );

    // GUILD COMMANDS
    let commands = guild_id.set_application_commands(&ctx, vec![
      commands::debug::register(),
      commands::rolepicker::register()
    ])
    .await;

    println!("I now have the following guild slash commands: {:#?}", commands);
    
    let stat = "Direktnachrichten für Hilfe";
    ctx.set_activity(Some(ActivityData::listening(stat)));
  }

  async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
    println!("Cache built successfully!");

    let ctx = Arc::new(ctx);

    let guild_id = GuildId::new(
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
          tokio::time::sleep(Duration::from_secs(601)).await;
        }
      });

      self.is_loop_running.swap(true, Ordering::Relaxed);
    }
  }
}

#[tokio::main]
async fn main() {

  // Use env file if available (for local testing only!)
  if dotenv::from_filename("dcs-dach-bot.env").ok().is_some() {
    println!(".env found and applied");
  }

  let token = env::var("DISCORD_TOKEN").expect("token");
  let intents = 
      GatewayIntents::DIRECT_MESSAGES
    | GatewayIntents::GUILDS
    | GatewayIntents::GUILD_MESSAGES
    // Priviledged
    | GatewayIntents::GUILD_MEMBERS
    | GatewayIntents::GUILD_PRESENCES;

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
