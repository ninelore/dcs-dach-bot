use indexmap::IndexMap;
use serenity::all::{ResolvedValue, ComponentInteraction, CommandInteraction, Role, ComponentInteractionDataKind, RoleId};
use serenity::{prelude::Context, model::Permissions};
use serenity::builder::{CreateSelectMenuOption, CreateCommand, CreateCommandOption, CreateEmbed, CreateSelectMenu, CreateInteractionResponseMessage, CreateInteractionResponse, CreateSelectMenuKind, EditMessage};

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
  for (key, val) in roles {
    options.push(CreateSelectMenuOption::new(val.first().unwrap(), &key));
  }

  let _picker = command
    .create_response(&ctx, CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
      .content("")
        .embed(CreateEmbed::new()
          .title(format!("Rollenwahl: {}", name))
          .field("Wähle deine Module", format!(" "), false)
        )
      .select_menu(CreateSelectMenu::new("rolepicker".to_string(), CreateSelectMenuKind::String { options: options }))
    ))
    .await
    .expect("Err while sending rolepicker");
}

pub async fn interaction(ctx: &Context, component: &ComponentInteraction) {
  let gid = component.clone().guild_id.unwrap();
  let guild = ctx.http.get_guild(gid).await.unwrap();
  let mut user = guild.member(&ctx, component.user.id).await.unwrap();

  let mut role_keys_cur: Vec<String> = Vec::new();
  for (key, val) in get_all() {
    let role_parsed: Role = {
      let role = val.first().unwrap();
      let guild = ctx.cache.guild(gid).unwrap();
      if guild.role_by_name(role).is_none() { continue; }
      let role_parse = guild.role_by_name(role).unwrap().to_owned();
      role_parse
    };
    if component.clone().user.has_role(ctx, gid, role_parsed).await.unwrap() {
      role_keys_cur.push(key);
    }
  }

  let sel_roles = match component.clone().data.kind {
    ComponentInteractionDataKind::StringSelect { values } => values,
    _ => vec!["Error: unexpected interaction caught".to_string()],
  };

  let mut role_ids_add: Vec<RoleId> = Vec::new();
  let mut role_ids_del: Vec<RoleId> = Vec::new();
  let mut role_ids_cat_add: Vec<RoleId> = Vec::new();
  let mut role_ids_cat_del: Vec<RoleId> = Vec::new();
  for role in sel_roles.clone() {
    let roleid_parsed: RoleId = {
      if guild.role_by_name(get_all().get_key_value(&role).unwrap().1.first().unwrap()).is_none() { continue; }
      let role_parse = guild.role_by_name(get_all().get_key_value(&role).unwrap().1.first().unwrap()).unwrap().to_owned();
      role_parse.id
    };
    if role_keys_cur.contains(&role) {
      role_ids_del.push(roleid_parsed);
      for add_role in get_all().get_key_value(&role).unwrap().1.clone() {
        if add_role.eq(&role) { continue; }
        let roleid_parsed: RoleId = {
          if guild.role_by_name(get_all().get_key_value(&add_role).unwrap().1.first().unwrap()).is_none() { continue; }
          let role_parse = guild.role_by_name(get_all().get_key_value(&add_role).unwrap().1.first().unwrap()).unwrap().to_owned();
          role_parse.id
        };
        if !user.roles.contains(&roleid_parsed) && !role_ids_cat_add.contains(&roleid_parsed) {
          role_ids_cat_del.push(roleid_parsed)
        }
      }
    } else {
      role_ids_add.push(roleid_parsed);
      for add_role in get_all().get_key_value(&role).unwrap().1.clone() {
        if add_role.eq(&role) { continue; }
        let roleid_parsed: RoleId = {
          if guild.role_by_name(get_all().get_key_value(&add_role).unwrap().1.first().unwrap()).is_none() { continue; }
          let role_parse = guild.role_by_name(get_all().get_key_value(&add_role).unwrap().1.first().unwrap()).unwrap().to_owned();
          role_parse.id
        };
        if !user.roles.contains(&roleid_parsed) && !role_ids_cat_add.contains(&roleid_parsed) {
          role_ids_cat_add.push(roleid_parsed)
        }
      }
    }
  }

  for add_role in role_ids_cat_del.clone() {
    if role_ids_cat_add.contains(&add_role) {
      role_ids_cat_del.retain(|p| {p.ne(&add_role)});
      role_ids_cat_add.retain(|p| {p.ne(&add_role)})
    }
  }

  let len = role_keys_cur.len() + role_ids_add.len() - role_ids_del.len();
  while len > 10 {
    role_ids_add.pop();
  }

  let mut err = false;
  if !role_ids_del.is_empty() {
    if let Err(why) = user.remove_roles(ctx, role_ids_del.clone().as_slice()).await {
      println!("Error while adding roles: {}", why);
      err = true
    } // */
  }
  if !role_ids_add.is_empty() {
    if let Err(why) = user.add_roles(ctx, role_ids_add.clone().as_slice()).await {
      println!("Error while adding roles: {}", why);
      err = true
    }
  } // */
  
  if !err {
    component.create_response(ctx, CreateInteractionResponse::Acknowledge).await.expect("Err while acknowledging ComponentInteraction");
    component.clone().message.edit(ctx, EditMessage::new().content("")).await.expect("Err while editing rolepicker");
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