use serenity::all::{CommandInteraction, CommandOptionType, Context, MessageId};
use serenity::builder::{
  CreateCommand, CreateCommandOption, CreateEmbed, CreateInteractionResponse,
  CreateInteractionResponseMessage,
};
use serenity::model::Permissions;
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ViewPollError {
  #[error("parsing error: `{0}`")]
  Parse(#[from] ParseIntError),
  #[error("message not found")]
  NotFound,
}

pub async fn view_poll(ctx: &Context, command: &CommandInteraction) -> Result<(), ViewPollError> {
  let react_icons = vec![
    "1️⃣".to_string(),
    "2️⃣".to_string(),
    "3️⃣".to_string(),
    "4️⃣".to_string(),
    "5️⃣".to_string(),
    "6️⃣".to_string(),
  ];

  let cid = command.clone().data.options[0]
    .value
    .as_channel_id()
    .expect("defined by the gateway");

  let mid = command.clone().data.options[1]
    .value
    .as_str()
    .expect("defined by the gateway")
    .to_string();

  let mid = mid.parse::<u64>();
  let mid: MessageId = match mid {
    Ok(mid) => mid.into(),
    Err(e) => {
      error_msg(&ctx, &command).await;
      return Err(ViewPollError::Parse(e));
    }
  };

  let msg = match ctx.http.get_message(cid, mid).await {
    Ok(msg) => msg,
    Err(_) => {
      error_msg(&ctx, &command).await;
      return Err(ViewPollError::NotFound);
    }
  };

  let mut data: Vec<(String, String, u64)> = Vec::new();
  for (i, reaction) in msg.reactions.iter().enumerate() {
    data.push((
      react_icons[i].clone(),
      msg.embeds.first().unwrap().fields[i].clone().name,
      reaction.count,
    ))
  }

  let mut data_sorted: Vec<_> = data.iter().collect();
  data_sorted.sort_by(|a, b| a.2.cmp(&b.2).reverse());

  let max = data_sorted.first().unwrap().2;
  let mut embedfields: Vec<(String, String, bool)> = vec![];
  for item in data_sorted {
    embedfields.push((
      item.1.to_owned(),
      progress_bar(item.2 - 1, max - 1, 20),
      false,
    ))
  }

  command
    .create_response(
      &ctx,
      CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new().embed(
          CreateEmbed::new()
            .title("Unfrageergebnis")
            .description(msg.embeds.first().unwrap().title.as_ref().unwrap())
            .fields(embedfields.clone()),
        ),
      ),
    )
    .await
    .expect("API down or missing permissions?");
  Ok(())
}

async fn error_msg(ctx: &Context, command: &CommandInteraction) {
  command
    .create_response(
      &ctx,
      CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
          .ephemeral(true)
          .content("Ungültige Nachichten-ID!"),
      ),
    )
    .await
    .unwrap();
}

fn progress_bar(progress: u64, max: u64, len: usize) -> String {
  let ratio = progress as f64 / max as f64;
  let filled = (len as f64 * ratio) as usize;
  "█".repeat(filled) + &"▒".repeat(len - filled)
}

pub fn register() -> CreateCommand {
  CreateCommand::new("view-poll")
    .description("Erzeuge Umfrageergebnisse")
    .default_member_permissions(Permissions::MANAGE_MESSAGES)
    .add_option(
      CreateCommandOption::new(
        CommandOptionType::Channel,
        "channel",
        "Kanal mit der Umfrage",
      )
      .required(true),
    )
    .add_option(
      CreateCommandOption::new(CommandOptionType::String, "messageid", "ID der Nachicht")
        .required(true),
    )
}
