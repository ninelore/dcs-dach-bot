use serenity::{prelude::Context, model::{prelude::{GuildId}, Permissions}, builder::CreateApplicationCommand};

pub fn run(ctx: &Context, _gid: GuildId) -> String {
  let map = ctx.cache.private_channels().into_iter();
  let len = ctx.cache.private_channels().len();
  for (key, value) in map {
    println!("{} / {}", key, value);
  }
  return format!("Hashmap size: {}", len);
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
  command.name("debug").description("Debug!").default_member_permissions(Permissions::ADMINISTRATOR)
}