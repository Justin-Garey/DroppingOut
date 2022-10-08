use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::channel::{Channel, ChannelType};
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption,
    CommandDataOptionValue,
};

pub fn run(options: &[CommandDataOption]) -> String {
    let option = options
        .get(0)
        .expect("Expected channel option")
        .resolved
        .as_ref()
        .expect("Expected channel object");

    if let CommandDataOptionValue::Channel(channel) = option {
        match channel.kind {
            ChannelType::Voice | 
            ChannelType::Stage  => format!("{}'s id is {}", channel.name.as_ref().expect("expected channel to have a name."), channel.id),
            _                   => "Select a voice channel".to_string(),
        }
    } else {
        "Please provide a valid channel".to_string()
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("disconnect").description("Disconnect from a channel").create_option(|option| {
        option
            .name("channel")
            .description("The user to lookup")
            .kind(CommandOptionType::Channel)
            .required(true)
    })
}