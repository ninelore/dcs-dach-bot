use std::env;

use serenity::{model::{prelude::{ChannelId, Message, interaction::{Interaction, InteractionResponseType}}, Timestamp}, prelude::Context};

pub async fn alert_moderators(ctx: &Context, msg: Message) {
  if let Err(why) = msg.reply_ping(&ctx.http, "Danke, dass Du uns kontaktiert hast. Dein Anliegen wird bearbeitet. \n \nThank you for contacting us. Your request is being processed.").await {
    println!("Error while answering DM: {:?}", why);
  }
  mod_announcement(&ctx, msg).await;
}

async fn mod_announcement(ctx: &Context, oldmsg: Message) {
  let cid = ChannelId::from(env::var("CHANNELID_MOD")
    .expect("channelid_mod is missing")
    .parse::<u64>()
    .expect("channelid_mod is no integer"));

  let _msg = cid
    .send_message(&ctx, |m| {
      m.content("@Moderator")
        .embed(|e| {
          e.title(format!("Anfrage von {}", oldmsg.author.name))
            .fields(vec![
              ("Name und ID", format!("<@{id}> - {id}", id = oldmsg.author.id), false),
              ("Nachricht", oldmsg.content, false),
            ])
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


  match interaction.clone().message_component().unwrap().data.custom_id.as_str() {
  "bearbeiten" | "takeover" => interaction.message_component().unwrap()
    .create_interaction_response(&ctx, |ed| {
      ed.kind(InteractionResponseType::UpdateMessage).interaction_response_data(|d| {
        d.content("@Moderator")
          .embed(|e| {
            e.title(format!("Anfrage von {}", oldmsg.author.name))
              .fields(vec![
                ("Name und ID", format!("<@{id}> - {id}", id = oldmsg.author.id), false),
                ("Nachricht", oldmsg.content, false),
                ("Bearbeiter", format!("<@{}>", bearbeiter), false)
              ])
              .footer(|f| f.text(format!("{},{}", oldmsg.channel_id.as_u64(), oldmsg.id.as_u64())))
              .timestamp(Timestamp::now())
          })
          .components(|c| {
            c.create_action_row(|row| {
              row.create_button(|b1| {
                b1.custom_id("takeover").label("Übernehmen").style(serenity::model::prelude::component::ButtonStyle::Primary)
              });
              row.create_button(|b2| {
                b2.custom_id("freigeben").label("Freigeben").style(serenity::model::prelude::component::ButtonStyle::Secondary)
              })
            })
          })
        })
      })
    .await.expect("Error while editing for interaction"),
  "freigeben" => interaction.message_component().unwrap()
    .create_interaction_response(&ctx, |ed| {
      ed.kind(InteractionResponseType::UpdateMessage).interaction_response_data(|d| {
        d.content("@Moderator")
        .embed(|e| {
          e.title(format!("Anfrage von {}", oldmsg.author.name))
            .fields(vec![
              ("Name und ID", format!("<@{id}> - {id}", id = oldmsg.author.id), false),
              ("Nachricht", oldmsg.content, false),
            ])
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
    })
    .await.expect("Error while editing for interaction"),
  _ => ()
  }
}
