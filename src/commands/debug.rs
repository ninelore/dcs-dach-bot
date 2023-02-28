use serenity::{prelude::Context, model::{prelude::{interaction::Interaction}, Permissions}, builder::CreateApplicationCommand};

pub async fn run(ctx: &Context, interaction: &Interaction) {
  let _m = interaction.clone().application_command().unwrap().channel_id
  .send_message(ctx, |m| {
    m.content("Test1")
  })
  .await
  .expect("Error in command: debug");
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
  command.name("debug").description("Debug!").default_member_permissions(Permissions::ADMINISTRATOR)
}