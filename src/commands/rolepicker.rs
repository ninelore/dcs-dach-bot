use serenity::all::ResolvedValue;
use serenity::{prelude::Context, all::Interaction, model::Permissions};
use serenity::builder::{CreateSelectMenuOption, CreateCommand, CreateCommandOption, CreateEmbed, CreateSelectMenu, CreateInteractionResponseMessage, CreateInteractionResponse, CreateSelectMenuKind};

use crate::util::consts::get_models;

pub async fn create_picker(ctx: &Context, interaction: &Interaction) -> Result<(), serenity::Error> {
  //if interaction.clone().application_command().unwrap().data.options().first().is_some() {
    match interaction.clone().application_command().unwrap().data.options().first().unwrap().value {
      ResolvedValue::Integer(1) => model_picker(ctx, interaction).await,
      _ => model_picker(ctx, interaction).await,
    }
  //} else {
  //  return model_picker(ctx, interaction).await
  //}
}

async fn model_picker(ctx: &Context, interaction: &Interaction) -> Result<(), serenity::Error> {
  let models = get_models();
  let mut options: Vec<CreateSelectMenuOption> = Vec::new();
  for (key, _val) in models {
    options.push(CreateSelectMenuOption::new(&key, &key));
    println!("add {}", &key)
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
      .add_int_choice("Module", 1)
      .add_int_choice("Andere", 2)
    ])
}