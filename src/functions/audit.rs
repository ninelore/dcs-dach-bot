use std::env;

use serenity::all::{ChannelId, Context, MessageBuilder, VoiceState};

pub async fn log_voice_audit(ctx: &Context, old: Option<VoiceState>, new: VoiceState) {
  let audit_cid = ChannelId::new(
    env::var("CHANNELID_AUDIT")
      .expect("CHANNELID_AUDIT is missing")
      .parse::<u64>()
      .expect("CHANNELID_AUDIT is no integer"),
  );

  if new.channel_id.is_none() {
    return;
  };

  let old_cid = match old {
    Some(old) => match old.channel_id {
      Some(old_cid) => old_cid.get(),
      _ => 0,
    },
    _ => 0,
  };

  if old_cid
    == new
      .channel_id
      .expect("Audit: Schrödingers new.channel_id")
      .get()
  {
    return;
  }

  let usr = new.user_id.to_user(&ctx).await.expect("Audit: User fehlt");

  let msg = MessageBuilder::new()
    .push(format!("{} ({}) hat ", usr.tag(), new.user_id.get()))
    .channel(new.channel_id.unwrap())
    .push(" betreten")
    .build();

  audit_cid.say(&ctx, msg).await.unwrap();
}
