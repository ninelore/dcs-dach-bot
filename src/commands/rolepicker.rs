use serenity::all::{ResolvedValue, ComponentInteraction, CommandInteraction, Role, ComponentInteractionDataKind, RoleId};
use serenity::{prelude::Context, model::Permissions};
use serenity::builder::{CreateSelectMenuOption, CreateCommand, CreateCommandOption, CreateEmbed, CreateSelectMenu, CreateInteractionResponseMessage, CreateInteractionResponse, CreateSelectMenuKind, EditMessage};

use crate::util::consts::*;

pub async fn create_picker(ctx: &Context, command: &CommandInteraction) {
  match command.data.options().first().unwrap().value {
    ResolvedValue::Integer(1) => role_picker(ctx, command, "Flaming Cliffs 3".to_string(), get_fc()).await,
    ResolvedValue::Integer(2) => role_picker(ctx, command, "Hubschrauber".to_string(), get_helis()).await,
    ResolvedValue::Integer(3) => role_picker(ctx, command, "Propellerflugzeuge".to_string(), get_props()).await,
    ResolvedValue::Integer(4) => role_picker(ctx, command, "Strahlflugzeuge".to_string(), get_jets()).await,
    _ => role_picker(ctx, command, "Andere Rollen".to_string(), get_other()).await,
  }
}

async fn role_picker(ctx: &Context, command: &CommandInteraction, name: String, roles: Vec<RoleOption>) {
  let mut options: Vec<CreateSelectMenuOption> = Vec::new();
  for roleop in roles {
    options.push(CreateSelectMenuOption::new(roleop.name, roleop.id));
  }

  let _picker = command
    .create_response(&ctx, CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
      .content("")
        .embed(CreateEmbed::new()
          .title(format!("Rollenwahl: {}", name))
          .field("Wähle deine Rollen", format!("Um eine Rolle zu entfernen, wähle sie erneut aus"), false)
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

  // Get Users current roles as const IDs
  let mut roles_cur: Vec<RoleOption> = Vec::new();
  for roleop in get_all() {
    let role_parsed: Role = {
      let guild = ctx.cache.guild(gid).unwrap();
      if guild.role_by_name(&roleop.name).is_none() { continue; }
      let role_parse = guild.role_by_name(&roleop.name).unwrap().to_owned();
      role_parse
    };
    if component.clone().user.has_role(ctx, gid, role_parsed).await.unwrap() {
      roles_cur.push(roleop);
    }
  }

  // Selected Roles to const IDs
  let sel_roles = match component.clone().data.kind {
    ComponentInteractionDataKind::StringSelect { values } => values,
    _ => vec!["Error: unexpected interaction caught".to_string()],
  };

  let mut role_ids_add: Vec<RoleId> = Vec::new();
  let mut role_ids_del: Vec<RoleId> = Vec::new();
  let mut role_ids_cat_add: Vec<RoleId> = Vec::new();
  let mut role_ids_cat_del: Vec<RoleId> = Vec::new();

  // Other Roles' own logic
  if sel_roles.first().is_some() && get_other().iter().any(|ro| &ro.id == sel_roles.first().unwrap()) {
    let roleid_parsed: RoleId = {
      if guild.role_by_name(&get_other().iter().find( |ro| &ro.id == sel_roles.first().unwrap()).unwrap().name).is_none() { return; }
      let role_parse = guild.role_by_name(&get_other().iter().find( |ro| &ro.id == sel_roles.first().unwrap()).unwrap().name).unwrap().to_owned();
      role_parse.id
    };
    if user.roles.contains(&roleid_parsed) {
      role_ids_del.push(roleid_parsed);
    } else {
      role_ids_add.push(roleid_parsed);
    }
  
    if !role_ids_del.is_empty() {
      role_ids_del.dedup();
      if let Err(why) = user.remove_roles(ctx, role_ids_del.clone().as_slice()).await {
        println!("Error while removing roles: {}", why);
      } // */
    }
    if !role_ids_add.is_empty() {
      role_ids_add.dedup();
      if let Err(why) = user.add_roles(ctx, role_ids_add.clone().as_slice()).await {
        println!("Error while adding roles: {}", why);
      }
    }
    success(&ctx, &component).await;
    return;
  }
  
  // Parse selected role to RoleID vectors
  for role in sel_roles.clone() {
    let roleid_parsed: RoleId = {
      if guild.role_by_name(&get_all().iter().find( |ro| &ro.id == sel_roles.first().unwrap()).unwrap().name).is_none() { continue; }
      let role_parse = guild.role_by_name(&get_all().iter().find( |ro| &ro.id == sel_roles.first().unwrap()).unwrap().name).unwrap().to_owned();
      role_parse.id
    };
    if roles_cur.iter().any(|p| p.id == role) {
      role_ids_del.push(roleid_parsed);
      for add_role in get_all().iter().find(|p| p.id == role).unwrap().add_role_names.clone().unwrap() { 
        let add_roleid_parsed: RoleId = {
          if guild.role_by_name(&add_role).is_none() { continue; }
          let role_parse = guild.role_by_name(&add_role).unwrap().to_owned();
          role_parse.id
        };
        if user.roles.contains(&add_roleid_parsed) {
          let mut remove = true;
          for orole in &roles_cur {
            if orole.id != role && orole.add_role_names.as_ref().unwrap().contains(&add_role) {
              remove = false
            }
          }
          if remove == true {
            role_ids_cat_del.push(add_roleid_parsed)
          }
        }
      }
    } else {
      role_ids_add.push(roleid_parsed);
      for add_role in get_all().iter().find(|p| p.id == role).unwrap().add_role_names.clone().unwrap() {
        let add_roleid_parsed: RoleId = {
          if guild.role_by_name(&add_role).is_none() { continue; }
          let role_parse = guild.role_by_name(&add_role).unwrap().to_owned();
          role_parse.id
        };
        if !user.roles.contains(&add_roleid_parsed) {
          role_ids_cat_add.push(add_roleid_parsed)
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

  if guild.role_by_name("Pilot").is_some() {
    let roleid_pilot: RoleId = {
      let role_parse = guild.role_by_name("Pilot").unwrap().to_owned();
      role_parse.id
    };
    if roles_cur.is_empty() {
      role_ids_cat_add.push(roleid_pilot)
    } else if roles_cur.len().eq(&role_ids_del.len()) {
      role_ids_cat_del.push(roleid_pilot)
    }
  }

  let mut lenerr = false;
  let len = roles_cur.len() + role_ids_add.len() - role_ids_del.len();
  while len > 10 {
    role_ids_add.pop();
    lenerr = true
  }

  let mut err = String::new();
  if lenerr {
    err = "Du hast zu viele Modulrollen gewählt".to_string();
  }
  if !role_ids_del.is_empty() {
    role_ids_del.extend(role_ids_cat_del);
    role_ids_del.dedup();
    if let Err(why) = user.remove_roles(ctx, role_ids_del.clone().as_slice()).await {
      println!("Error while removing roles: {}", why);
    } // */
  }
  if !role_ids_add.is_empty() {
    role_ids_add.extend(role_ids_cat_add);
    role_ids_add.dedup();
    if let Err(why) = user.add_roles(ctx, role_ids_add.clone().as_slice()).await {
      println!("Error while adding roles: {}", why);
    }
  } // */
  
  if err.is_empty() {
    success(&ctx, &component).await
  } else {
    send_error(&ctx, &component, err).await
  }
}

async fn success(ctx: &Context, component: &ComponentInteraction) {
  component.create_response(ctx, CreateInteractionResponse::Acknowledge).await.expect("Err while acknowledging ComponentInteraction");
  component.clone().message.edit(ctx, EditMessage::new().content("")).await.expect("Err while editing rolepicker");
}

async fn send_error(ctx: &Context, component: &ComponentInteraction, msg: String) {
  component.create_response(ctx, CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
    .ephemeral(true)
    .content(msg)
  ))
  .await
  .expect("Error while sending error message")
}

pub fn register() -> CreateCommand {
  CreateCommand::new("role")
    .description("Erstelle eine Rollenzuweisung")
    .default_member_permissions(Permissions::MANAGE_CHANNELS)
    .set_options(vec![CreateCommandOption::new(serenity::all::CommandOptionType::Integer, "typ".to_string(), "Welche Rollenzuweisung?".to_string())
      .required(true)
      .add_int_choice("Flaming Cliffs 3", 1)
      .add_int_choice("Hubschrauber", 2)
      .add_int_choice("Propellerflugzeuge", 3)
      .add_int_choice("Strahlflugzeuge", 4)
      .add_int_choice("Sonstige", 0)
    ])
}