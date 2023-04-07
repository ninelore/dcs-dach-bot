use serenity::all::{CommandInteraction, CommandOptionType, Context, CreateCommand, Permissions};
use serenity::builder::{CreateCommandOption, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage};

pub async fn create_poll(ctx: &Context, command: &CommandInteraction) {
  let mut frage: String = String::new();
  let mut answers: Vec<String> = vec![];
  let mut answerfields: Vec<(String, String, bool)> = vec![];
  for i in command.clone().data.options {
    match i.name.clone().as_str() {
      "frage" => frage = format!("Umfrage: {}", i.value.as_str().unwrap().to_string()),
      _ => {
        if i.value.clone().as_str().is_some() {
          answers.push(i.value.clone().as_str().unwrap().to_string());
        }
      }
    }
  }
  for i in answers {
    answerfields.push((i, String::new(), false))
  }

  command.create_response(&ctx, CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
    .embed(CreateEmbed::new()
      .title(frage)
      .fields(answerfields)
    )
  )).await.expect("Err while sending message");
}

pub fn register() -> CreateCommand {
  CreateCommand::new("poll")
    .description("Erstelle eine Umfrage")
    .default_member_permissions(Permissions::KICK_MEMBERS)
    .add_option(CreateCommandOption::new(CommandOptionType::String, "frage", "Stelle eine Frage").required(true))
    .add_option(CreateCommandOption::new(CommandOptionType::String, "antwort-1", "Antwortmöglichkeit 1").required(true))
    .add_option(CreateCommandOption::new(CommandOptionType::String, "antwort-2", "Antwortmöglichkeit 2").required(true))
    .add_option(CreateCommandOption::new(CommandOptionType::String, "antwort-3", "Antwortmöglichkeit 3").required(false))
    .add_option(CreateCommandOption::new(CommandOptionType::String, "antwort-4", "Antwortmöglichkeit 4").required(false))
    .add_option(CreateCommandOption::new(CommandOptionType::String, "antwort-5", "Antwortmöglichkeit 5").required(false))
    .add_option(CreateCommandOption::new(CommandOptionType::String, "antwort-6", "Antwortmöglichkeit 6").required(false))
}