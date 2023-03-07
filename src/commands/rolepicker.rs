use std::env;

use indexmap::IndexMap;
use serenity::all::{ResolvedValue, ComponentInteraction, CommandInteraction, GuildId, Role, ComponentInteractionDataKind};
use serenity::{prelude::Context, model::Permissions};
use serenity::builder::{CreateSelectMenuOption, CreateCommand, CreateCommandOption, CreateEmbed, CreateSelectMenu, CreateInteractionResponseMessage, CreateInteractionResponse, CreateSelectMenuKind};

use crate::util::consts::*;

pub async fn create_picker(ctx: &Context, command: &CommandInteraction) {
  match command.data.options().first().unwrap().value {
    ResolvedValue::Integer(1) => role_picker(ctx, command, "Jets".to_string(), get_jets()).await,
    ResolvedValue::Integer(2) => role_picker(ctx, command, "Helikopter".to_string(), get_helis()).await,
    ResolvedValue::Integer(3) => role_picker(ctx, command, "Propellerflugzeuge".to_string(), get_props()).await,
    ResolvedValue::Integer(4) => role_picker(ctx, command, "Low-fidelity / Flaming Cliffs".to_string(), get_fc()).await,
    _ => role_picker(ctx, command, "Andere Rollen".to_string(), get_jets()).await, //TODO: Other Roles
  }
}

async fn role_picker(ctx: &Context, command: &CommandInteraction, name: String, roles: IndexMap<String, Vec<String>>) {
  let mut options: Vec<CreateSelectMenuOption> = Vec::new();
  for (key, _val) in roles {
    options.push(CreateSelectMenuOption::new(_val.first().unwrap(), &key));
  }

  let _picker = command
    .create_response(&ctx, CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
      .content("")
        .embed(CreateEmbed::new()
          .title(format!("Rollenwahl: {}", name))
          .field("Wähle deine Module", format!(" "), false)
        )
      .select_menu(CreateSelectMenu::new("rolepicker".to_string(), CreateSelectMenuKind::String { options: options })
        .max_values(10)
      )
    ))
    .await
    .expect("something went wrong at module_picker");
}

pub async fn interaction(ctx: &Context, component: &ComponentInteraction) {
  let guild_id = GuildId::new(
    env::var("GUILD_ID")
      .expect("Expected GUILD_ID in environment")
      .parse()
      .expect("GUILD_ID must be an integer"),
  );

  let mut count = 0;
  for (_key, val) in get_all() {
    for role in &val {
      let role_parsed: Role = {
        let guild = ctx.cache.guild(guild_id).unwrap();
        let role_parse = guild.role_by_name(role).expect("Err: Role not found").to_owned();
        role_parse
      };
      if component.user.has_role(&ctx, guild_id, role_parsed).await.unwrap() {
        count += 1;
      }
    }
  }
  if count >= 10 {
    let sel_roles = match component.clone().data.kind {
      ComponentInteractionDataKind::StringSelect { values } => values,
      _ => vec!["Error: unexpected interaction caught".to_string()],
    };
  }
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