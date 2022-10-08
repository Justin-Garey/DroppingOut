use serenity::client::Context;
use serenity::model::id::GuildId;
use serenity::builder::CreateApplicationCommand;

pub async fn run(ctx: &Context, guild_id: GuildId) -> String {
    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();
    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        if let Err(e) = manager.remove(guild_id).await {
            return format!("Failed: {:?}", e);
        }
    } else {
        return "Not in a voice channel".to_string();
    }

    "Left voice channel".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("disconnect").description("Disconnect from a channel")
}
