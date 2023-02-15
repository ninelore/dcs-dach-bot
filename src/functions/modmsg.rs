use serenity::{model::{prelude::{ReactionType, Message}, Timestamp}, prelude::Context};

pub async fn alert_moderators(ctx: &Context, msg: Message) {
  if let Err(why) = msg.react(&ctx.http, ReactionType::Unicode(String::from_utf16(&[0x2705]). unwrap())).await {
    println!("Error while reacting to DM: {:?}", why);
  }
  if let Err(why) = msg.reply_ping(&ctx.http, "Vielen Dank für deine Nachicht! Ein Moderator wird sich in Kürze bei dir melden!").await {
    println!("Error while answering DM: {:?}", why);
  }
  mod_announcement(&ctx, msg).await;
}

async fn mod_announcement(ctx: &Context, oldmsg: Message) {
  let msg = oldmsg
    .channel_id
    .send_message(&ctx.http, |m| {
      m.content("@Moderator")
          .embed(|e| {
              e.title(format!("Supportanfrage von {}", oldmsg.author.name))
                  .description("This is a description")
                  .fields(vec![
                      ("Voller Name und ID", format!("<@{id}> - {id}", id = oldmsg.author.id), false),
                      ("Nachicht", oldmsg.content, true),
                  ])
                  .footer(|f| f.text("Erhalten via DM an den Bot"))
                  .timestamp(Timestamp::now())
          })
  })
  .await;

  if let Err(why) = msg {
    println!("Error sending message: {:?}", why);
  }   
}
