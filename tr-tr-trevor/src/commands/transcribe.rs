
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use serenity::model::prelude::command::CommandOptionType;

pub fn run(_options: &[CommandDataOption]) -> String {
    "Hey, I'm alive!".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("transcribe")
        .description("Start transcription of a voice chat")
        .create_option( |option| {
            option
                .name("Voice Channel")
                .description("Voice Channel to connect to")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
