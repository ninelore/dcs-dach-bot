use serenity::{prelude::Context, all::Interaction, model::Permissions, builder::{CreateCommand, CreateMessage}};

pub async fn run(ctx: &Context, interaction: &Interaction) -> Result<(), serenity::Error> {
  let _m = interaction.clone().application_command().unwrap().channel_id
  .send_message(ctx, CreateMessage::new()
    .content("Test1")
  )
  .await
  .expect("Error in command: debug");
  Ok(())
}

pub fn register() -> CreateCommand {
  CreateCommand::new("debug").description("Debug!").default_member_permissions(Permissions::ADMINISTRATOR)
}