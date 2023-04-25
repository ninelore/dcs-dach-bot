use serenity::all::ReactionType::Unicode;
use serenity::all::{
  CommandInteraction, CommandOptionType, Context, CreateCommand, Permissions, RoleId,
};
use serenity::builder::{
  CreateAllowedMentions, CreateCommandOption, CreateEmbed, CreateInteractionResponse,
  CreateInteractionResponseMessage, GetMessages,
};

pub async fn create_poll(ctx: &Context, command: &CommandInteraction) {
  let mut frage: String = String::new();
  let mut answers: Vec<(String, String)> = vec![];
  let mut ping = RoleId::new(9223372036854775806);
  let mut everyone = false;
  let mut content = String::new();
  let mut answerfields: Vec<(String, String, bool)> = vec![];
  let react_icons = vec![
    "1️⃣".to_string(),
    "2️⃣".to_string(),
    "3️⃣".to_string(),
    "4️⃣".to_string(),
    "5️⃣".to_string(),
    "6️⃣".to_string(),
  ];

  for i in &command.data.options.clone() {
    match i.name.clone().as_str() {
      "frage" => frage = format!("Umfrage: {}", i.value.as_str().unwrap()),
      "everyone" => everyone = true,
      "ping" => ping = i.value.as_role_id().unwrap(),
      _ => {
        if let Some(value) = i.value.as_str() {
          answers.push((i.name.clone().as_str().to_string(), value.to_string()));
        }
      }
    }
  }

  for i in answers.clone() {
    let icon = react_icons[i.0.split('-').last().unwrap().parse::<usize>().unwrap() - 1].clone();
    answerfields.push((format!("{} | {}", icon, i.1), String::new(), false))
  }

  if everyone {
    content = "@everyone".to_string()
  } else if ping.to_role_cached(ctx).is_some() {
    content = format!("<@&{}>", ping.0)
  }

  command
    .create_response(
      &ctx,
      CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
          .content(content)
          .allowed_mentions(CreateAllowedMentions::new().all_roles(true).everyone(true))
          .embed(CreateEmbed::new().title(frage).fields(answerfields.clone())),
      ),
    )
    .await
    .expect("Err while sending message");

  let channel = command.channel_id.to_channel(&ctx).await.unwrap();
  let channel = channel.guild().unwrap();
  let poll = {
    let msgs = channel
      .messages(&ctx, GetMessages::new().limit(3))
      .await
      .unwrap();
    let poll = msgs
      .iter()
      .find(|p| p.author.id.0 == command.application_id.0)
      .expect("Couldnt find the message that i just sent...")
      .clone();
    poll
  };

  for i in answers.clone() {
    let num = i.0.split('-').last().unwrap().parse::<usize>().unwrap();
    let reaction_type = Unicode(react_icons[num - 1].clone());
    let poll = poll.react(&ctx, reaction_type).await;
    poll.expect("Err couldnt react");
  }
}

pub fn register() -> CreateCommand {
  CreateCommand::new("create-poll")
    .description("Erstelle eine Umfrage")
    .default_member_permissions(Permissions::MANAGE_MESSAGES)
    .add_option(
      CreateCommandOption::new(CommandOptionType::String, "frage", "Stelle eine Frage")
        .required(true),
    )
    .add_option(
      CreateCommandOption::new(
        CommandOptionType::String,
        "antwort-1",
        "Antwortmöglichkeit 1",
      )
      .required(true),
    )
    .add_option(
      CreateCommandOption::new(
        CommandOptionType::String,
        "antwort-2",
        "Antwortmöglichkeit 2",
      )
      .required(true),
    )
    .add_option(
      CreateCommandOption::new(
        CommandOptionType::String,
        "antwort-3",
        "Antwortmöglichkeit 3",
      )
      .required(false),
    )
    .add_option(
      CreateCommandOption::new(
        CommandOptionType::String,
        "antwort-4",
        "Antwortmöglichkeit 4",
      )
      .required(false),
    )
    .add_option(
      CreateCommandOption::new(
        CommandOptionType::String,
        "antwort-5",
        "Antwortmöglichkeit 5",
      )
      .required(false),
    )
    .add_option(
      CreateCommandOption::new(
        CommandOptionType::String,
        "antwort-6",
        "Antwortmöglichkeit 6",
      )
      .required(false),
    )
    .add_option(
      CreateCommandOption::new(
        CommandOptionType::Boolean,
        "everyone",
        "Soll @everyone gepingt werden? (Überschreibt 'ping'!)",
      )
      .required(false),
    )
    .add_option(
      CreateCommandOption::new(
        CommandOptionType::Role,
        "ping",
        "Soll eine Rolle gepingt werden?",
      )
      .required(false),
    )
}
