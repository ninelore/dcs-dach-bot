use serenity::model::voice::VoiceState;
use serenity::model::prelude::{GuildId, ChannelId, ChannelType, Channel};
use serenity::prelude::Context;

pub async fn members(ctx: &Context, gid: GuildId) {
  let count = ctx.cache.guild(gid).unwrap().members.len();
  if let Err(why) = ChannelId(1067584919188029522).edit(&ctx.http, |c| c.name(format!("Mitglieder: {:?}", count))).await {
    println!("Error while renaming: {:?}", why);
  } else {
    println!("Rename apparently successful");
  }
}

pub async fn members_voice(ctx: &Context,  _old: Option<VoiceState>, _vs: VoiceState) {
  let mut count = 0;
  let channels: Vec<Channel> = ctx.cache.clone().guild(_vs.guild_id.unwrap()).unwrap().channels.into_values().collect();
  for ch in channels {
    if ch.clone().guild().is_some() {
      if ch.clone().guild().expect("Err at channel type check").kind == ChannelType::Voice {
        count = count + ch.guild().unwrap().members(&ctx.cache).await.unwrap().len();
      }
    }
  }
  if let Err(why) = ChannelId(1067593373835350100).edit(&ctx.http, |c| c.name(format!("Mitglieder im Voice: {:?}", count))).await {
    println!("Error while renaming: {:?}", why);
  } else {
    println!("Rename apparently successful");
  }
}
