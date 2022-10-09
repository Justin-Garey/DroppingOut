mod commands;
// use crate::commands::*;

use std::env;

use songbird::{
    driver::DecodeMode,
    model::payload::{ClientDisconnect, Speaking},
    Config,
    Event,
    EventContext,
    EventHandler as VoiceEventHandler,
    SerenityInit,
};

use serenity::{
    client::{Client, EventHandler, Context},
};

use serenity::async_trait;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::id::{GuildId, ChannelId};
use serenity::prelude::*;
use serenity::http::client::Http;

use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::time::*;

pub fn getTranscribedText() -> String {

    let mut rv: String = String::new();
    if Path::new("message.txt").exists() {
        fs::copy("message.txt", "messaged.txt");
        {
            let mut fd = File::open("messaged.txt").expect("couldnt open file we know exists...");
            eprintln!("FUCKING file descriptor: \'{:?}\'", fd);
            let _ = fd.read_to_string(&mut rv);
        }
        fs::remove_file("message.txt");

    } else {
        return String::new();
    }
    
    // let files = fs::read_dir("./").unwrap();
    // for file in files {
    //     let f = file.unwrap().path().file_name().unwrap().to_str().unwrap().to_string();
    //     // println!("{}",f);
    //     if f.contains("message.txt") {
    //         fs::copy("message.txt", "messaged.txt"); 

    //         let mut tmp: String = String::new();
    //         {
    //             let mut fd = File::open("messaged.txt").expect("couldnt open file we know exists...");
    //             eprintln!("FUCKING file descriptor: \'{:?}\'", fd);
    //             let str = fd.read_to_string(&mut tmp);

    //         }
    //         eprintln!("FUCKING tmp: \'{}\'", tmp);
    //         rv = [rv, tmp].concat();
    //         eprintln!("FUCKING rv: \'{}\'", rv);
    //         fs::remove_file("message.txt");
    //     } 
    // }

    return rv;
}

struct Receiver;

impl Receiver {
    pub fn new() -> Self {
        // You can manage state here, such as a buffer of audio packet bytes so
        // you can later store them in intervals.
        Self { }
    }
}

#[async_trait]
impl VoiceEventHandler for Receiver {
    #[allow(unused_variables)]
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        use EventContext as Ctx;
        match ctx {
            Ctx::SpeakingStateUpdate(
                Speaking {speaking, ssrc, user_id, ..}
            ) => {
                // Discord voice calls use RTP, where every sender uses a randomly allocated
                // *Synchronisation Source* (SSRC) to allow receivers to tell which audio
                // stream a received packet belongs to. As this number is not derived from
                // the sender's user_id, only Discord Voice Gateway messages like this one
                // inform us about which random SSRC a user has been allocated. Future voice
                // packets will contain *only* the SSRC.
                //
                // You can implement logic here so that you can differentiate users'
                // SSRCs and map the SSRC to the User ID and maintain this state.
                // Using this map, you can map the `ssrc` in `voice_packet`
                // to the user ID and handle their audio packets separately.
                println!(
                    "Speaking state update: user {:?} has SSRC {:?}, using {:?}",
                    user_id,
                    ssrc,
                    speaking,
                );
            },
            Ctx::SpeakingUpdate(data) => {
                // You can implement logic here which reacts to a user starting
                // or stopping speaking, and to map their SSRC to User ID.
                println!(
                    "Source {} has {} speaking.",
                    data.ssrc,
                    if data.speaking {"started"} else {"stopped"},
                );

                if !data.speaking {
                    let unix_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                    fs::rename(data.ssrc.to_string(), format!("{}_processing_{}.pcm", unix_time, data.ssrc))
                        .expect("Couldn't rename user's file.");
                }
            },
            Ctx::VoicePacket(data) => {
                // An event which fires for every received audio packet,
                // containing the decoded data.
                if let Some(audio) = data.audio {
                    println!("Audio packet's first 5 samples: {:?}", audio.get(..5.min(audio.len())));
                    // println!(
                    //     "Audio packet sequence {:05} has {:04} bytes (decompressed from {}), SSRC {}",
                    //     data.packet.sequence.0,
                    //     audio.len() * std::mem::size_of::<i16>(),
                    //     data.packet.payload.len(),
                    //     data.packet.ssrc,
                    // );
                    let mut file: File = File::options().append(true).create(true).open(data.packet.ssrc.to_string()).unwrap();
                    let raw_audio = data.audio.clone();
                    let audio_byte_tuples = raw_audio.unwrap().into_iter().map(|x| { x.to_le_bytes() });
                    let audio_bytes: Vec<u8> = audio_byte_tuples.flatten().collect();
                    file.write_all(&audio_bytes).unwrap();
                } else {
                    println!("RTP packet, but no audio. Driver may not be configured to decode.");
                }
            },
            Ctx::RtcpPacket(data) => {
                // An event which fires for every received rtcp packet,
                // containing the call statistics and reporting information.
                // println!("RTCP packet received: {:?}", data.packet);
                let channel_id = ChannelId(1028322765599682592);

                let http = Http::new_with_application_id(
                    &env::var("DISCORD_TOKEN").expect("Expected Discord token"),
                    env::var("APP_ID").expect("Expected Application ID").parse::<u64>().unwrap(),
                );

                let tmp = getTranscribedText();

                let msg = channel_id.send_message(&http, |m| {
                    m.content(tmp.clone())
                }).await;

                println!("WHY ARENT YOU PRINTING {} TO {:?}", tmp, channel_id);

                fs::remove_file("message.txt");
            },
            Ctx::ClientDisconnect(
                ClientDisconnect {user_id, ..}
            ) => {
                // You can implement your own logic here to handle a user who has left the
                // voice channel e.g., finalise processing of statistics etc.
                // You will typically need to map the User ID to their SSRC; observed when
                // first speaking.

                println!("Client disconnected: user {:?}", user_id);
            },
            _ => {
                // We won't be registering this struct for any more event classes.
                unimplemented!()
            }
        }

        None
    }
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        if let Interaction::ApplicationCommand(command) = interaction {
            // println!("Received command interaction: {:#?}", command);


            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content("Working...".to_string()))
                }).await
            {
                println!("Cannot respond to slash command: {}", why);
            }

            let content = match command.data.name.as_str() {
                "ping" => commands::ping::run(&command.data.options),
                "transcribe" => commands::transcribe::run(&ctx, guild_id, &command.data.options).await,
                "disconnect" => commands::disconnect::run(&ctx, guild_id).await,
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .edit_original_interaction_response(&ctx.http, |response| {
                    response.content(content)
                }).await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let _commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::ping::register(command))
                .create_application_command(|command| commands::transcribe::register(command))
                .create_application_command(|command| commands::disconnect::register(command))
        })
        .await;

        // println!("I now have the following guild slash commands: {:#?}", commands);

    }
}


#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::MESSAGE_CONTENT;

    // Set up Songbird to decode audio packets
     let songbird_config = Config::default()
        .decode_mode(DecodeMode::Decode);

    // Build our client.
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .register_songbird_from_config(songbird_config)
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}


async fn print_transcript(message: String) {

}
