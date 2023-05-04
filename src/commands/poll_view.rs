use serenity::all::{CommandInteraction, CommandOptionType, Context, Message};
use serenity::builder::{
  CreateCommand, CreateCommandOption, CreateEmbed, CreateInteractionResponse,
  CreateInteractionResponseMessage,
};
use serenity::model::Permissions;

pub async fn view_poll(ctx: &Context, command: &CommandInteraction) {
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
    .unwrap();
  let mid_str = {
    let t = command.clone().data.options[1]
      .value
      .as_str()
      .unwrap()
      .to_string();
    t
  };

  let mid = mid_str.parse::<u64>();
  let msg: Message;
  let msg_t = ctx.cache.message(cid, mid.clone().unwrap());
  if mid.is_ok() {
    if msg_t.is_some() {
      msg = msg_t.unwrap()
    } else {
      error_msg(&ctx, &command).await;
      println!("message anhand id nicht gefunden: {}", mid.unwrap());
      return;
    }
  } else {
    println!("mid parsing err");
    error_msg(&ctx, &command).await;
    return;
  }

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
    embedfields.push((item.1.to_owned(), progress_bar(item.2, max), false))
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

fn progress_bar(progress: u64, max: u64) -> String {
  if progress == max {
    return format!("####################");
  }
  let satz_dec = format!("{:.2}", progress / max).parse::<f64>().unwrap();
  let barsize = (20.0 * satz_dec) as usize;
  let bar = "#".repeat(barsize)
    + "-"
      .repeat((max - (20.0 * satz_dec) as u64) as usize)
      .as_str();
  return bar;
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
