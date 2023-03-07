use std::collections::HashMap;

use serenity::all::ResolvedValue;
use serenity::{prelude::Context, all::Interaction, model::Permissions};
use serenity::builder::{CreateSelectMenuOption, CreateCommand, CreateCommandOption, CreateEmbed, CreateSelectMenu, CreateInteractionResponseMessage, CreateInteractionResponse, CreateSelectMenuKind};

use crate::util::consts::*;

pub async fn create_picker(ctx: &Context, interaction: &Interaction) -> Result<(), serenity::Error> {
  match interaction.clone().application_command().unwrap().data.options().first().unwrap().value {
    ResolvedValue::Integer(1) => role_picker(ctx, interaction, get_jets()).await,
    ResolvedValue::Integer(2) => role_picker(ctx, interaction, get_helis()).await,
    ResolvedValue::Integer(3) => role_picker(ctx, interaction, get_props()).await,
    ResolvedValue::Integer(4) => role_picker(ctx, interaction, get_fc()).await,
    _ => role_picker(ctx, interaction, get_jets()).await, //TODO: Other Roles
  }
}

async fn role_picker(ctx: &Context, interaction: &Interaction, roles: HashMap<String, Vec<String>>) -> Result<(), serenity::Error> {
  let mut options: Vec<CreateSelectMenuOption> = Vec::new();
  for (key, _val) in roles {
    options.push(CreateSelectMenuOption::new(_val.first().unwrap(), &key));
  }

  let _picker = interaction.clone().application_command().unwrap()
    .create_response(&ctx, CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
      .content("")
        .embed(CreateEmbed::new()
          .title(format!("Rollenwahl: Module"))
          .field("Wähle deine Module", format!(" "), false)
        )
      .select_menu(CreateSelectMenu::new("moduleroles".to_string(), CreateSelectMenuKind::String { options: options })
        .max_values(10)
      )
    ))
    .await
    .expect("something went wrong at module_picker");
  Ok(())
}

pub fn register() -> CreateCommand {
  CreateCommand::new("rolepicker")
    .description("Create a rolepicker!")
    .default_member_permissions(Permissions::MANAGE_CHANNELS)
    .set_options(vec![CreateCommandOption::new(serenity::all::CommandOptionType::Integer, "typ".to_string(), "Was für ein Rolepicker?".to_string())
      .required(true)
      .add_int_choice("Module: Jets", 1)
      .add_int_choice("Module: Helis", 2)
      .add_int_choice("Module: Propellerflugzeuge", 3)
      .add_int_choice("Module: FC3", 4)
      .add_int_choice("Andere", 0)
    ])
}