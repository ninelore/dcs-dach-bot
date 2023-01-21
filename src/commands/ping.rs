use serenity::model::prelude::interaction::application_command::CommandDataOption;
use serenity::builder::CreateApplicationCommand;

pub fn run(_options: &[CommandDataOption]) -> String {
  "Hey, I'm alive!".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
  command.name("ping").description("A ping command")
}