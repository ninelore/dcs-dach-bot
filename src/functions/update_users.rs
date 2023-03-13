use std::env;

use serenity::builder::EditChannel;
use serenity::model::user::OnlineStatus;
use serenity::model::prelude::{GuildId, ChannelId};
use serenity::prelude::Context;

pub async fn members(ctx: &Context, gid: GuildId) {
  let cid = ChannelId::from(env::var("CHANNELID_ALL")
    .expect("channelid_all is missing")
    .parse::<u64>()
    .expect("channelid_all is no integer"));
  let count = ctx.cache.guild(gid).unwrap().member_count;
  if let Err(why) = cid.edit(&ctx.http, EditChannel::new().name(format!("Mitglieder: {:?}", count))).await {
    println!("Error while renaming: {:?}", why);
  } // */
}

pub async fn members_online(ctx: &Context, gid: GuildId) {
  let cid = ChannelId::from(env::var("CHANNELID_ONLINE")
    .expect("channelid_online is missing")
    .parse::<u64>()
    .expect("channelid_online is no integer"));
  let count2 = {
    let guild = ctx.cache.guild(gid).unwrap();
    let count2 = guild.members_with_status(OnlineStatus::Online).len() 
      + guild.members_with_status(OnlineStatus::Idle).len()
      + guild.members_with_status(OnlineStatus::DoNotDisturb).len();
      count2
  };
  if let Err(why) = cid.edit(&ctx.http, EditChannel::new().name(format!("Mitglieder Online: {:?}", count2))).await {
    println!("Error while renaming: {:?}", why);
  } // */
}
