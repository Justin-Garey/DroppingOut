use crate::{Receiver, GLOBAL_GUILD_TO_CHANNEL};
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
    let option = options
    .get(0)
    .expect("Expected channel option")
    .resolved
    .as_ref()
    .expect("Expected channel object");
    let outchannel = options
    .get(1)
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
            return "Please provide a valid Voice channel".to_string()
        };
    let txtchannel_id = 
        if let CommandDataOptionValue::Channel(channel) = outchannel {
            match channel.kind {
                ChannelType::Text           |
                ChannelType::Private        |
                ChannelType::PublicThread   | 
                ChannelType::PrivateThread   => channel.id,
                _                 => return "Select an output Text channel".to_string(),
            }
        } else {
            return "Please provide a valid Text channel".to_string()
        };
    println!("setting cell of channel to {}", txtchannel_id);
    GLOBAL_GUILD_TO_CHANNEL.with(|cell_of_channel| {
        *cell_of_channel.borrow_mut() = txtchannel_id.clone();
        println!("cell of channel is now {:?}", cell_of_channel);
    });

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


    format!("Transcription of channel <#{}> to <#{}>begun.", channel_id, txtchannel_id)
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
                .name("output_channel")
                .description("Text Channel to transcribe to")
                .kind(CommandOptionType::Channel)
                .required(true)
        })
}
