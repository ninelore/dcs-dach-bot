use std::{env, time::SystemTime};

use serenity::{model::{prelude::{ChannelId, Message, interaction::{Interaction, InteractionResponseType}}, Timestamp}, prelude::Context};

pub async fn alert_moderators(ctx: &Context, msg: Message) {

  let timeout: i64 = 7200;

  let ch = msg.channel(&ctx).await.unwrap().private().unwrap();
  let bmsg = ch.messages(&ctx, |b| b).await.unwrap().into_iter()
    .find(|p| p.author.id == ctx.cache.current_user().id && p.content.starts_with("Danke"));
  let mut lastrequest = 0;
  if bmsg.is_some() {
    lastrequest = bmsg.unwrap().timestamp.unix_timestamp();
  }
 
  if SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("Time error").as_secs() < (lastrequest + timeout).try_into().unwrap() {
    if let Err(why) = msg.reply(&ctx.http, "(PH) Error Spam attempt").await {
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

  let cid = ChannelId::from(env::var("CHANNELID_MOD")
    .expect("channelid_mod is missing")
    .parse::<u64>()
    .expect("channelid_mod is no integer"));

  let _msg = cid
    .send_message(&ctx, |m| {
      m.content("<@&691859336561164300>")
        .embed(|e| {
          e.title(format!("Anfrage von {}", oldmsg.author.name))
            .fields(fields)
            .field("Bearbeiter", format!(" "), true)
            .field("Status", "Offen", true)
            .footer(|f| f.text(format!("{},{}", oldmsg.channel_id.as_u64(), oldmsg.id.as_u64())))
            .timestamp(Timestamp::now())
        })
        .components(|c| {
          c.create_action_row(|row| {
            row.create_button(|b1| {
              b1.custom_id("bearbeiten").label("Bearbeiten").style(serenity::model::prelude::component::ButtonStyle::Primary)
            })
          })
        })
    })
  .await
  .expect("Error sending message");
}

pub async fn interaction(ctx: &Context, interaction: Interaction) {
  let msg = interaction.clone().message_component().expect("Error: message component").message;
  let footerdata: Vec<&str> = msg.embeds.first().unwrap().footer.as_ref().unwrap().text.split(",").collect();
  let oldmsg = ctx.http.get_message(footerdata[0].parse::<u64>().unwrap(), footerdata[1].parse::<u64>().unwrap()).await.expect("Err: old msg not found");
  let bearbeiter = interaction.clone().message_component().unwrap().member.unwrap().user.id.0;

  let fields = vec![
    ("Benutzername", format!("<@{}>", oldmsg.author.id), true),
    ("Benutzer-ID", format!("{}", oldmsg.author.id), true),
    ("Nachricht", oldmsg.content, false)
  ];

  match interaction.clone().message_component().unwrap().data.custom_id.as_str() {
  "bearbeiten" | "takeover" => interaction.clone().message_component().unwrap()
    .create_interaction_response(&ctx, |ed| {
      ed.kind(InteractionResponseType::UpdateMessage).interaction_response_data(|d| {
        d.content("<@&691859336561164300>")
          .embed(|e| {
            e.title(format!("Anfrage von {}", oldmsg.author.name))
              .fields(fields)
              .field("Bearbeiter", format!("<@{}>", bearbeiter), true)
              .field("Status", "In Bearbeitung", true)
              .footer(|f| f.text(format!("{},{}", oldmsg.channel_id.as_u64(), oldmsg.id.as_u64())))
              .timestamp(Timestamp::now())
          })
          .components(|c| {
            c.create_action_row(|row| {
              row.create_button(|b2| {
                b2.custom_id("freigeben").label("Freigeben").style(serenity::model::prelude::component::ButtonStyle::Secondary)
              });
              row.create_button(|b3| {
                b3.custom_id("close").label("Schließen").style(serenity::model::prelude::component::ButtonStyle::Danger)
              })
            })
          })
        })
      })
    .await.expect("Error while editing for interaction"),
  "freigeben" => interaction.message_component().unwrap()
    .create_interaction_response(&ctx, |ed| {
      ed.kind(InteractionResponseType::UpdateMessage).interaction_response_data(|d| {
        d.content("<@&691859336561164300>")
        .embed(|e| {
          e.title(format!("Anfrage von {}", oldmsg.author.name))
            .fields(fields)
            .field("Bearbeiter", format!(" "), true)
            .field("Status", "Offen", true)
            .footer(|f| f.text(format!("{},{}", oldmsg.channel_id.as_u64(), oldmsg.id.as_u64())))
            .timestamp(Timestamp::now())
        })
        .components(|c| {
          c.create_action_row(|row| {
            row.create_button(|b1| {
              b1.custom_id("bearbeiten").label("Bearbeiten").style(serenity::model::prelude::component::ButtonStyle::Primary)
            });
            row.create_button(|b2| {
              b2.custom_id("close").label("Schließen").style(serenity::model::prelude::component::ButtonStyle::Danger)
            })
          })
        })
      })
    })
    .await.expect("Error while editing for interaction"),
  "close" => interaction.message_component().unwrap()
    .create_interaction_response(&ctx, |ed| {
      ed.kind(InteractionResponseType::UpdateMessage).interaction_response_data(|d| {
        d.content("Geschlossenes Ticket")
        .embed(|e| {
          e.title(format!("Anfrage von {}", oldmsg.author.name))
            .fields(fields)
            .field("Bearbeiter", format!("<@{}>", bearbeiter), true)
            .field("Status", "Geschlossen", true)
            .footer(|f| f.text(format!("{},{}", oldmsg.channel_id.as_u64(), oldmsg.id.as_u64())))
            .timestamp(Timestamp::now())
        })
        .components(|c| {
          c.create_action_row(|row| {
            row.create_button(|b1| {
              b1.custom_id("bearbeiten").label("Geschlossen").style(serenity::model::prelude::component::ButtonStyle::Danger).disabled(true)
            })
          })
        })
      })
    })
    .await.expect("Error while editing for interaction"),
  _ => ()
  }
}
