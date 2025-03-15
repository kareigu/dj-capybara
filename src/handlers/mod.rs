use crate::{commands, shared::ConfigStorage};
use serenity::{
    builder::CreateInteractionResponseMessage,
    model::{application::CommandInteraction, prelude::Ready},
    prelude::Context,
};
use tracing::{error, info};

pub mod playback;
pub mod utils;
pub use utils::text_response;

static COMMAND_TIMEOUT: tokio::time::Duration = tokio::time::Duration::from_secs(10);

pub async fn register_commands(ctx: &Context, _ready: &Ready) {
    let config_lock = {
        let data = ctx.data.read().await;
        data.get::<ConfigStorage>()
            .expect("No config in global storage")
            .clone()
    };

    let commands = if let Some(guild) = config_lock.guild_id {
        guild
            .set_commands(&ctx.http, commands::command_list())
            .await
    } else {
        ctx.http
            .create_global_commands(&commands::command_list())
            .await
    };

    match commands {
        Ok(c) => {
            let cmd_list = c.iter().fold("".to_string(), |mut a, c| {
                let s = format!("{}\n", c.name);
                a.push_str(&s);
                a
            });
            if let Some(guild) = config_lock.guild_id {
                info!("Added commands for Guild({}):\n{}", guild, cmd_list)
            } else {
                info!("Added global commands:\n{}", cmd_list)
            }
        }
        Err(e) => panic!(
            "Couldn't set {} application commands: {:#?}",
            e,
            if config_lock.guild_id.is_some() {
                ""
            } else {
                "global"
            }
        ),
    };
}

pub async fn handle_commands(ctx: &Context, command: CommandInteraction) {
    let name = &command.data.name;
    let user = &command.user;
    match command
        .create_response(
            &ctx.http,
            serenity::builder::CreateInteractionResponse::Defer(
                CreateInteractionResponseMessage::new().content("Loading"),
            ),
        )
        .await
    {
        Ok(_) => info!("{} command deferred", name),
        Err(e) => error!("Error deferring command {}: {}", name, e),
    }

    let result = commands::execute(name, ctx, &command);

    match tokio::time::timeout(COMMAND_TIMEOUT, result).await {
        Ok(result) => {
            if let Err(e) = result {
                error!("Couldn't respond to command: {}", e);
                error!(
                    "{user} failed running command {cmd}",
                    user = user.tag(),
                    cmd = name
                );
                text_response(ctx, &command, "Error processing command")
                    .await
                    .unwrap_or(());
            } else {
                info!("{user} ran command {cmd}", user = user.tag(), cmd = name)
            }
        }
        Err(e) => {
            error!("Couldn't respond to command: {}", e);
            error!(
                "{user} failed running command {cmd}",
                user = user.tag(),
                cmd = name
            );
            text_response(ctx, &command, "Took too long processing command")
                .await
                .unwrap_or(());
        }
    }
}
