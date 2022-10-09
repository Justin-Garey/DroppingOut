use crate::{Receiver, EventHandler};
use songbird::{CoreEvent};
use serenity::async_trait;

use serenity::client::Context;
use serenity::model::id::GuildId;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption,
    CommandDataOptionValue,
};
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::channel::ChannelType;

// maybe make channel autoselection? aka. join the sending user's voice channel when unspecified
pub async fn run(ctx: &Context, guild_id: GuildId, options: &[CommandDataOption]) -> String {
    let option = options
    .get(0)
    .expect("Expected channel option")
    .resolved
    .as_ref()
    .expect("Expected channel object");

    let channel_id = 
        if let CommandDataOptionValue::Channel(channel) = option {
            match channel.kind {
                ChannelType::Voice | 
                ChannelType::Stage  => channel.id,//format!("{}'s id is {}", channel.name.as_ref().expect("expected channel to have a name."), channel.id),
                _                   => return "Select a voice channel".to_string(),
            }
        } else {
            return "Please provide a valid channel".to_string()
        };
    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();
    let (handler_lock, conn_result) = manager.join(guild_id, channel_id).await;

    if let Ok(_) = conn_result {
        
        let mut handler = handler_lock.lock().await;
        handler.add_global_event(
            CoreEvent::SpeakingStateUpdate.into(),
            Receiver::new(),
        );

        handler.add_global_event(
            CoreEvent::SpeakingUpdate.into(),
            Receiver::new(),
        );

        handler.add_global_event(
            CoreEvent::VoicePacket.into(),
            Receiver::new(),
        );

        handler.add_global_event(
            CoreEvent::RtcpPacket.into(),
            Receiver::new(),
        );

        handler.add_global_event(
            CoreEvent::ClientDisconnect.into(),
            Receiver::new(),
        );
    }


    format!("Transcription of channel <#{}> begun.", channel_id)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("transcribe")
        .description("Start transcription of a voice chat")
        .create_option( |option| {
            option
                .name("voice_channel")
                .description("Voice Channel to connect to")
                .kind(CommandOptionType::Channel)
                .required(true)
        })
}
