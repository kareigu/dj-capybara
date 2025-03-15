use crate::{
    commands::Command,
    handlers::{playback::VOIPData, utils::text_response},
};
use serenity::{
    Error, async_trait, builder::CreateCommand, client::Context,
    model::application::CommandInteraction,
};
use tracing::error;

pub struct Stop;

#[async_trait]
impl Command for Stop {
    async fn execute(ctx: &Context, command: &CommandInteraction) -> Result<(), Error> {
        let voip_data = match VOIPData::from(ctx, command).await {
            Ok(v) => v,
            Err(s) => return text_response(ctx, command, s).await,
        };

        let guild_id = voip_data.guild_id;

        let manager = match songbird::get(ctx).await {
            Some(arc) => arc.clone(),
            None => {
                error!("Error with songbird client");
                return text_response(ctx, command, "Error getting voice client").await;
            }
        };

        let handler_lock = match manager.get(guild_id) {
            Some(h) => h,
            None => return text_response(ctx, command, "Not in a voice channel").await,
        };

        let handler = handler_lock.lock().await;
        handler.queue().stop();

        text_response(ctx, command, "Stopped playback and cleared the queue").await
    }

    const NAME: &'static str = "stop";

    fn info() -> CreateCommand {
        CreateCommand::new(Self::NAME).description("Stop music and clear the queue")
    }
}
