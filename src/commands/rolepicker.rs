//use crate::util::consts;

use serenity::{prelude::Context, model::{prelude::{interaction::Interaction}, Permissions}, builder::{CreateApplicationCommand, CreateSelectMenuOption}};

pub async fn create_picker(ctx: &Context, interaction: &Interaction) {
  let models = crate::util::consts::get_models();
  let mut options: Vec<CreateSelectMenuOption> = Vec::new();
  for (key, _val) in models {
    options.push(CreateSelectMenuOption::new(&key, &key))
  }

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
              s.custom_id("rolepicker")
                .max_values(10)
                .options(|o| {
                  o.set_options(options)
                })
            })
          })
        })
    })
    .await
    .expect("Error in command: create_picker");
  
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
  command.name("rolepicker")
    .description("Create a rolepicker!")
    .default_member_permissions(Permissions::MANAGE_CHANNELS)
    .create_option(|o| {
      o.name("typ")
        .description("Was für ein Rolepicker?")
        .kind(serenity::model::prelude::command::CommandOptionType::Integer)
        .add_int_choice("Module", 1)
        .add_int_choice("Andere", 2)
  })
}