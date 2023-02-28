//use crate::util::consts;

use serenity::{prelude::Context, model::{prelude::{GuildId, interaction::Interaction}, Permissions}, builder::CreateApplicationCommand};

pub fn create_picker(ctx: &Context, interaction: &Interaction, gid: GuildId) -> String {
  let _picker = interaction.clone().application_command().unwrap().channel_id
    .send_message(&ctx, |m| {
      m.content("")
        .embed(|e| {
          e.title(format!("Rollenwahl: Module"))
            .field("Wähle deine Module", format!(" "), false)
            .footer(|f| f.text(format!("")))
        })
        .components(|c| {
          c.create_action_row(|row| {
            row.create_select_menu(|s| {
              s.disabled(true)
            })
          })
        })
    });

  let _roles = ctx.cache.guild_roles(gid).expect("Error while retrieving roles");
  
  return "Test".to_string();
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
  command.name("rolepicker").description("Create a rolepicker!").default_member_permissions(Permissions::MANAGE_CHANNELS)
    .create_option(|o| {
      o.name("Typ")
       .kind(serenity::model::prelude::command::CommandOptionType::String)
       .add_int_choice("Module", 1)
       .add_int_choice("Andere", 2)
  })
}