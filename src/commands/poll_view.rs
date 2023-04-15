use serenity::all::{CommandInteraction, CommandOptionType, Context};
use serenity::builder::{
  CreateCommand, CreateCommandOption, CreateInteractionResponse, CreateInteractionResponseMessage,
};
use serenity::model::Permissions;

pub async fn view_poll(ctx: &Context, command: &CommandInteraction) {
  let link = command.data.options.first().unwrap();
  let link = link.value.as_str().unwrap().to_string();
  let ids = {
    let chunks: Vec<_> = link.split('/').collect();
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

  assert_eq!(ids.len(), 2);
  dbg!(&ids);

  todo!()
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
