use serenity::{
  all::CommandInteraction,
  builder::{CreateCommand, CreateMessage},
  model::Permissions,
  prelude::Context,
};

pub async fn run(ctx: &Context, command: &CommandInteraction) {
  let _m = command
    .channel_id
    .send_message(ctx, CreateMessage::new().content("Test1"))
    .await
    .expect("Error in command: debug");
}

pub fn register() -> CreateCommand {
  CreateCommand::new("debug")
    .description("Debug!")
    .default_member_permissions(Permissions::ADMINISTRATOR)
}
