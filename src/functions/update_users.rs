use std::env;

use serenity::model::user::OnlineStatus;
use serenity::model::prelude::{GuildId, ChannelId};
use serenity::prelude::Context;

pub async fn members(ctx: &Context, gid: GuildId) {
  let cid = env::var("CHANNELID_ALL").expect("channelid_all").parse::<u64>().expect("channelid_all: u64");
  let count = ctx.cache.guild(gid).unwrap().member_count;
  if let Err(why) = ChannelId(cid).edit(&ctx.http, |c| c.name(format!("Mitglieder: {:?}", count))).await {
    println!("Error while renaming: {:?}", why);
  } // */
}

pub async fn members_online(ctx: &Context, gid: GuildId) {
  let cid = env::var("CHANNELID_ONLINE").expect("channelid_online").parse::<u64>().expect("channelid_online: u64");
  let guild = ctx.cache.guild(gid).unwrap();
  let count2 = guild.members_with_status(OnlineStatus::Online).len() 
    + guild.members_with_status(OnlineStatus::Idle).len()
    + guild.members_with_status(OnlineStatus::DoNotDisturb).len();
  if let Err(why) = ChannelId(cid).edit(&ctx.http, |c| c.name(format!("Mitglieder Online: {:?}", count2))).await {
    println!("Error while renaming: {:?}", why);
  } // */
}
