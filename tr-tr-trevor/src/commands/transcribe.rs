use crate::{Receiver, CHANNEL_ID_ARC};
use songbird::CoreEvent;

use serenity::{
    client::Context,
    builder::CreateApplicationCommand,
    model::{
        id::GuildId,
        channel::ChannelType,
        prelude::{
            command::CommandOptionType,
            interaction::application_command::{
                CommandDataOption,
                CommandDataOptionValue,
            },
        }
    },
};

use std::process::Command;

// maybe make channel autoselection? aka. join the sending user's voice channel when unspecified
pub async fn run(ctx: &Context, guild_id: GuildId, options: &[CommandDataOption]) -> String {  
    let option1 = options
        .get(0)
        .expect("Expected channel option")
        .resolved
        .as_ref()
        .expect("Expected channel object");

    let channel_id = 
        if let CommandDataOptionValue::Channel(channel) = option1 {
            match channel.kind {
                ChannelType::Voice | 
                ChannelType::Stage  => channel.id,//format!("{}'s id is {}", channel.name.as_ref().expect("expected channel to have a name."), channel.id),
                _                   => return "Select a voice channel".to_string(),
            }
        } else {
            return "Please provide a valid channel".to_string()
        };

    let option2 = options
        .get(1)
        .expect("Expected channel option")
        .resolved
        .as_ref()
        .expect("Expected channel object");

    let text_channel_id = 
        if let CommandDataOptionValue::Channel(channel) = option2 {
            match channel.kind {
                ChannelType::Text | ChannelType::Private | ChannelType::PrivateThread |
                ChannelType::PublicThread  => channel.id,
                _                   => return "Select a text channel".to_string(),
            }
        } else {
            return "Please provide a valid channel".to_string()
        };
    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();
    let (handler_lock, conn_result) = manager.join(guild_id, channel_id).await;
    if let Ok(_) = conn_result {
        
        let mut handler = handler_lock.lock().await;

        Command::new("./whisper_tr_tr.sh").spawn().expect("expected whisper_tr_tr to spawn...");

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
    
    if let Ok(mut locked_channel_id_ref) = CHANNEL_ID_ARC.lock() {
        *locked_channel_id_ref = text_channel_id;
        println!("now outputting text to channel {}", text_channel_id.as_u64());
    }

    format!("Transcription of channel <#{}> to <#{}> begun.", channel_id, text_channel_id)
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
        .create_option( |option| {
            option
                .name("text_channel")
                .description("Text Channel to transcribe to")
                .kind(CommandOptionType::Channel)
                .required(true)
        })
}
