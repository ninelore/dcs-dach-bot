use serenity::model::prelude::interaction::application_command::CommandDataOption;
use serenity::model::prelude::Message;
use serenity::builder::CreateApplicationCommand;
use serenity::prelude::Context;
use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::command;

#[command] 
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;
    Ok(())
}

pub fn run(_options: &[CommandDataOption]) -> String {
  "Hey, I'm alive!".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
  command.name("ping").description("A ping command")
}