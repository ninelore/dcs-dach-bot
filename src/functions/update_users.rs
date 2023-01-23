use serenity::{prelude::Context, model::prelude::{GuildId, ChannelId}};

pub async fn run(ctx: &Context, gid: GuildId) {
  let count = ctx.cache.guild(gid).unwrap().members.len();
  if let Err(why) = ChannelId(1028632535418290257).edit(&ctx, |c| c.name(format!("Mitglieder: {:?}", count))).await {
    println!("Error while renaming: {:?}", why);
  } else {
    println!("Das sollte geklappt haben...")
  }
}
