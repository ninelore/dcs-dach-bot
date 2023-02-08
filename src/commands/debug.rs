use serenity::{prelude::Context, model::{prelude::{GuildId}, user::OnlineStatus}, builder::CreateApplicationCommand};

pub fn run(ctx: &Context, gid: GuildId) -> String {
  let count = ctx.cache.guild(gid).unwrap().members_with_status(OnlineStatus::Offline).len();
  return format!("Members online: {}", count);
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
  command.name("debug").description("Debug!")
}