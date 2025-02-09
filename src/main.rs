mod commands;
mod functions;
mod handler;
mod util;

use serenity::all::GuildId;
use serenity::prelude::GatewayIntents;
use serenity::Client;
use std::env;

#[tokio::main]
async fn main() {
  // Use env file if available (for local testing only!)
  if dotenv::from_filename("dcs-dach-bot.env").is_ok() {
    println!(".env found and applied!");
  }

  let token = env::var("DISCORD_TOKEN").expect("the token should be present");
  let intents = GatewayIntents::DIRECT_MESSAGES
    | GatewayIntents::GUILDS
    | GatewayIntents::GUILD_MESSAGES
    | GatewayIntents::GUILD_VOICE_STATES
    // Privileged
    | GatewayIntents::GUILD_MEMBERS
    | GatewayIntents::GUILD_PRESENCES;

  let guild_id = GuildId::new(
    env::var("GUILD_ID")
      .expect("Expected GUILD_ID in environment")
      .parse()
      .expect("GUILD_ID must be an integer"),
  );

  let mut client = Client::builder(token, intents)
    .event_handler(handler::Handler::new(guild_id))
    .await
    .expect("Error creating client");

  if let Err(why) = client.start().await {
    println!("Client error: {:?}", why);
  }
}
