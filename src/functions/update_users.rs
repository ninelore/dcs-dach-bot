use std::env;

use dotenv;

use serenity::model::user::OnlineStatus;
use serenity::model::prelude::{GuildId, ChannelId};
use serenity::prelude::Context;

pub async fn members(ctx: &Context, gid: GuildId) {
  dotenv::dotenv().ok();
  let cid = env::var("CHANNELID_ALL").expect("channelid_all").parse::<u64>().expect("channelid_all: u64");
  let count = ctx.cache.guild(gid).unwrap().member_count;
  println!("Fetched amount overall: {}", count);
  if let Err(why) = ChannelId(cid).edit(&ctx.http, |c| c.name(format!("Mitglieder: {:?}", count))).await {
    println!("Error while renaming: {:?}", why);
  } // */
}

pub async fn members_online(ctx: &Context, gid: GuildId) {
  let cid = env::var("CHANNELID_ONLINE").expect("channelid_online").parse::<u64>().expect("channelid_online: u64");
  let count_offline = ctx.cache.guild(gid).unwrap().members_with_status(OnlineStatus::Offline).len();
  let count_all = ctx.cache.guild(gid).unwrap().members.len();
  let count = count_all - count_offline;
  if let Err(why) = ChannelId(cid).edit(&ctx.http, |c| c.name(format!("Mitglieder Online: {:?}", count))).await {
    println!("Error while renaming: {:?}", why);
  }// */
}
