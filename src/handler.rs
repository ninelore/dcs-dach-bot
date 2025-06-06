use crate::commands;
use crate::functions;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use serenity::all::VoiceState;
use serenity::all::{GuildId, Interaction, Message, Ready};
use serenity::async_trait;
use serenity::gateway::ActivityData;
use serenity::prelude::{Context, EventHandler};

pub struct Handler {
  guild_id: GuildId,
  is_running: AtomicBool,
}

impl Handler {
  pub fn new(guild_id: GuildId) -> Self {
    Self {
      guild_id,
      is_running: AtomicBool::new(false),
    }
  }
}

#[async_trait]
impl EventHandler for Handler {
  async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
    println!("Cache built successfully!");

    let gid = self.guild_id;
    if !self.is_running.load(Ordering::Relaxed) {
      let ctx = Arc::new(ctx);
      tokio::spawn(async move {
        loop {
          functions::update_users::members(&ctx, gid).await;
          tokio::time::sleep(Duration::from_secs(3)).await;
          functions::update_users::members_online(&ctx, gid).await;
          tokio::time::sleep(Duration::from_secs(7201)).await;
        }
      });

      self.is_running.swap(true, Ordering::Relaxed);
    }
  }

  async fn message(&self, ctx: Context, msg: Message) {
    if msg.guild_id.is_none() && msg.author != **ctx.cache.current_user() {
      functions::modmsg::alert_moderators(&ctx, msg).await;
    }
  }

  async fn ready(&self, ctx: Context, ready: Ready) {
    println!("{} is connected!", ready.user.name);

    let gid = self.guild_id;
    // GUILD COMMANDS
    let commands = gid
      .set_commands(&ctx, vec![commands::debug::register()])
      .await;

    match commands {
      Ok(_) => println!("Commands registered successfully!"),
      Err(err) => println!("{err:#?}"),
    }

    let stat = "Erstelle ein Ticket, um das Team zu kontaktieren";
    ctx.set_activity(Some(ActivityData::playing(stat)));
  }

  #[allow(clippy::single_match)]
  async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
    match &interaction {
      Interaction::Command(command) => {
        match command.data.name.as_str() {
          "debug" => commands::debug::run(&ctx, command).await,
          _ => (),
        };
      }

      Interaction::Component(component) => match component.data.custom_id.as_str() {
        "assign" | "unassign" | "close" => functions::modmsg::interaction(&ctx, component).await,
        _ => (),
      },
      _ => println!("INFO: Unhandled Interaction caught"),
    }
  }

  async fn voice_state_update(&self, ctx: Context, old: Option<VoiceState>, new: VoiceState) {
    functions::audit::log_voice_audit(&ctx, old, new).await;
  }
}
