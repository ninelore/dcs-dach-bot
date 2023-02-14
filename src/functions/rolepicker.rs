use std::env;

use serenity::{prelude::Context, model::prelude::{GuildId, ChannelId}};

pub fn check_picker_exists(ctx: &Context, gid: GuildId) {
  let cid = env::var("CHANNELID_ROLES").expect("channelid_roles").parse::<u64>().expect("channelid_roles: u64");
  // TBD
  //ChannelId(cid).message(???).???
}

pub fn create_picker(ctx: &Context, gid: GuildId) {
  // TBD
}
