use serenity::{prelude::Context, model::{prelude::{GuildId, interaction::Interaction}, Permissions}, builder::CreateApplicationCommand};

pub fn run(ctx: &Context, interaction: &Interaction, _gid: GuildId) -> String {
  let _m = interaction.clone().application_command().unwrap().channel_id
  .send_message(ctx, |m| {
    m.content("Test1")
  });
  return "Echo".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
  command.name("debug").description("Debug!").default_member_permissions(Permissions::ADMINISTRATOR)
}