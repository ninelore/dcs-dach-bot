use serenity::all::{CommandInteraction, CommandOptionType, Context};
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
  let link = command.data.options.first().unwrap();
  let link_str = link.value.as_str().unwrap().to_string();
  let ids = {
    // TODO: Aus link_str kommt BS raus. Wieso???
    // Bsp: "https://discord.com/channels/1028451968588451920/1040597438597173279/1096193249904971826"
    let chunks: Vec<_> = link_str.split('/').collect();
    let ids = chunks.iter().rev().take(2);
    let ids: Vec<_> = ids.filter_map(|p| p.parse::<u64>().ok()).collect();
    ids
  };

  if ids.len() != 2 {
    command
      .create_response(
        &ctx,
        CreateInteractionResponse::Message(
          CreateInteractionResponseMessage::new()
            .ephemeral(true)
            .content("Ungültiger Nachichtenlink!"),
        ),
      )
      .await
      .unwrap();
    return;
  }

  let msg = ctx.cache.message(ids[0], ids[1]).unwrap();

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
    .expect("Couldnt create response. API down or missing permissions?");
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
        CommandOptionType::String,
        "umfragelink",
        "Nachichtenlink zur Umfrage",
      )
      .required(true),
    )
}
