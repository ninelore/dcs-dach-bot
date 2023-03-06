use std::{env, time::SystemTime};

use serenity::{prelude::Context, all::{Message, MessageId, ChannelId, Interaction}, builder::{GetMessages, CreateInteractionResponseMessage, CreateInteractionResponse, CreateMessage, CreateEmbed, CreateEmbedFooter, CreateButton}, model::Timestamp};

pub async fn alert_moderators(ctx: &Context, msg: Message) {

  let timeout: i64 = 7200;

  let ch = msg.channel(&ctx).await.unwrap().private().unwrap();
  let bmsg = ch.messages(&ctx, GetMessages::new()).await.unwrap().into_iter()
    .find(|p| p.author.id == ctx.cache.current_user().id && p.content.starts_with("Danke"));
  let mut lastrequest = 0;
  if bmsg.is_some() {
    lastrequest = bmsg.unwrap().timestamp.unix_timestamp();
  }
 
  if SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("Time error").as_secs() < (lastrequest + timeout).try_into().unwrap() {
    if let Err(why) = msg.reply(&ctx.http, "Bitte warte etwas bevor du noch eine Anfrage sendest. \n \nPlease wait some time before you try to send another request.").await {
      println!("Error while answering DM: {:?}", why);
    }
  } else {
    if let Err(why) = msg.reply_ping(&ctx.http, "Danke, dass Du uns kontaktiert hast. Deine Anfrage wird bearbeitet. \n \nThank you for contacting us. Your request is being processed.").await {
      println!("Error while answering DM: {:?}", why);
    }
    mod_announcement(&ctx, msg).await;
  }
}

async fn mod_announcement(ctx: &Context, oldmsg: Message) {
  let fields = vec![
    ("Benutzername", format!("<@{}>", oldmsg.author.id), true),
    ("Benutzer-ID", format!("{}", oldmsg.author.id), true),
    ("Nachricht", oldmsg.content, false)
  ];

  let cid = ChannelId::new(env::var("CHANNELID_MOD")
    .expect("channelid_mod is missing")
    .parse::<u64>()
    .expect("channelid_mod is no integer"));

  let _msg = cid
    .send_message(&ctx, CreateMessage::new()
      .content("<@&691859336561164300>")
        .embed(CreateEmbed::new()
          .title(format!("Anfrage von {}", oldmsg.author.name))
            .fields(fields)
            .field("Bearbeiter", format!(" "), true)
            .field("Status", "Offen", true)
            .footer(CreateEmbedFooter::new(format!("{},{}", oldmsg.channel_id, oldmsg.id)))
            .timestamp(Timestamp::now())
        )
        .button(CreateButton::new("bearbeiten").label("Bearbeiten").style(serenity::model::prelude::ButtonStyle::Primary))
    )
  .await
  .expect("Error sending message");
}

pub async fn interaction(ctx: &Context, interaction: &Interaction) {
  let msg = interaction.clone().message_component().expect("Error: message component").message;
  let footerdata: Vec<&str> = msg.embeds.first().unwrap().footer.as_ref().unwrap().text.split(",").collect();
  let oldmsg = ctx.http.get_message(ChannelId::new(footerdata[0].parse::<u64>().unwrap()), MessageId::new(footerdata[1].parse::<u64>().unwrap())).await.expect("Err: old msg not found");
  let bearbeiter = interaction.clone().message_component().unwrap().member.unwrap().user.id.0;

  let fields = vec![
    ("Benutzername", format!("<@{}>", oldmsg.author.id), true),
    ("Benutzer-ID", format!("{}", oldmsg.author.id), true),
    ("Nachricht", oldmsg.content, false)
  ];

  match interaction.clone().message_component().unwrap().data.custom_id.as_str() {
  "bearbeiten" | "takeover" => interaction.clone().message_component().unwrap()
    .create_response(&ctx, CreateInteractionResponse::UpdateMessage(CreateInteractionResponseMessage::new()
      .content("<@&691859336561164300>")
      .embed(CreateEmbed::new()
        .title(format!("Anfrage von {}", oldmsg.author.name))
          .fields(fields)
          .field("Bearbeiter", format!("<@{}>", bearbeiter), true)
          .field("Status", "In Bearbeitung", true)
          .footer(CreateEmbedFooter::new(format!("{},{}", oldmsg.channel_id, oldmsg.id)))
          .timestamp(Timestamp::now())
      )
      .button(CreateButton::new("freigeben").label("Freigeben").style(serenity::model::prelude::ButtonStyle::Secondary))
      .button(CreateButton::new("close").label("Schließen").style(serenity::model::prelude::ButtonStyle::Danger))
    ))
    .await.expect("Error while editing for interaction"),
  "freigeben" => interaction.clone().message_component().unwrap()
    .create_response(&ctx, CreateInteractionResponse::UpdateMessage(CreateInteractionResponseMessage::new()
    .content("<@&691859336561164300>")
      .embed(CreateEmbed::new()
        .title(format!("Anfrage von {}", oldmsg.author.name))
          .fields(fields)
          .field("Bearbeiter", format!(" "), true)
          .field("Status", "Offen", true)
          .footer(CreateEmbedFooter::new(format!("{},{}", oldmsg.channel_id, oldmsg.id)))
          .timestamp(Timestamp::now())
      )
      .button(CreateButton::new("bearbeiten").label("Bearbeiten").style(serenity::model::prelude::ButtonStyle::Primary))
      .button(CreateButton::new("close").label("Schließen").style(serenity::model::prelude::ButtonStyle::Danger))
    ))
    .await.expect("Error while editing for interaction"),
  "close" => interaction.clone().message_component().unwrap()
    .create_response(&ctx, CreateInteractionResponse::UpdateMessage(CreateInteractionResponseMessage::new()
      .content("Geschlossenes Ticket")
      .embed(CreateEmbed::new()
        .title(format!("Anfrage von {}", oldmsg.author.name))
          .fields(fields)
          .field("Bearbeiter", format!("<@{}>", bearbeiter), true)
          .field("Status", "Geschlossen", true)
          .footer(CreateEmbedFooter::new(format!("{},{}", oldmsg.channel_id, oldmsg.id)))
          .timestamp(Timestamp::now())
      )
      .button(CreateButton::new("bearbeiten").label("Geschlossen").style(serenity::model::prelude::ButtonStyle::Danger).disabled(true))
    ))
    .await.expect("Error while editing for interaction"),
  _ => ()
  }
}
