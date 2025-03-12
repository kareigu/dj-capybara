use crate::config::ConfigStorage;
use serenity::builder::CreateInteractionResponseMessage;
use serenity::model::application::CommandInteraction;
use serenity::model::prelude::Ready;
use serenity::prelude::Context;
use tracing::{error, info};

mod cmd;
mod playback;
mod utils;
use utils::text_response;

static COMMAND_TIMEOUT: tokio::time::Duration = tokio::time::Duration::from_secs(10);

pub async fn register_commands(ctx: &Context, _ready: &Ready) {
  let config_lock = {
    let data = ctx.data.read().await;
    data
      .get::<ConfigStorage>()
      .expect("No config in global storage")
      .clone()
  };

  if let Some(guild) = config_lock.guild_id {
    let commands = guild.set_commands(&ctx.http, cmd::command_list()).await;

    match commands {
      Ok(c) => {
        let cmd_list = c.iter().fold("".to_string(), |mut a, c| {
          let s = format!("{}\n", c.name);
          a.push_str(&s);
          a
        });
        info!("Added commands for Guild({}):\n{}", guild, cmd_list)
      }
      Err(e) => panic!("Couldn't set application commands: {:#?}", e),
    }
  } else {
    let commands = ctx.http.create_global_commands(&cmd::command_list()).await;

    match commands {
      Ok(c) => {
        let cmd_list = c.iter().fold("".to_string(), |mut a, c| {
          let s = format!("{}\n", c.name);
          a.push_str(&s);
          a
        });
        info!("Added global commands:\n{}", cmd_list)
      }
      Err(e) => panic!("Couldn't set global application commands: {:#?}", e),
    }
  }
}

pub async fn handle_commands(ctx: &Context, command: CommandInteraction) {
  let name = command.data.name.clone();
  let user = command.user.clone();
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

  let result = cmd::execute(name.as_str(), ctx, &command);

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
