use crate::commands::{Command, playback::{VOIPData, format_duration_live}, text_response};
use serenity::async_trait;
use serenity::client::Context;
use serenity::builder::CreateApplicationCommand;
use serenity::model::interactions::application_command::ApplicationCommandInteraction;
use tracing::error;
use serenity::Error;
use std::time::Duration;
use crate::constants::EMBED_COLOUR;

pub struct Skip;

#[async_trait]
impl Command for Skip {

  async fn execute(ctx: &Context, command: ApplicationCommandInteraction) -> Result<(), Error> {
    let voip_data = match VOIPData::from(ctx, &command).await {
      Ok(v) => v,
      Err(s) => return text_response(ctx, command, s).await
    };
  
    let guild_id = voip_data.guild_id;
    let channel_id = voip_data.channel_id;
  
    let manager = match songbird::get(ctx).await {
      Some(arc) => arc.clone(),
      None => {
        error!("Error with songbird client");
        return text_response(ctx, command, "Error getting voice client".to_string()).await
      }
    };
  
    let handler_lock = match manager.get(guild_id) {
      Some(h) => h,
      None => {
        let join = manager.join(guild_id, channel_id).await;
        match join.1 {
          Ok(_) => join.0,
          Err(e) => {
            error!("Error joining voice channel: {}", e);
            return text_response(ctx, command, "Not in a voice channel".to_string()).await
          }
        }
      }
    };

    let handler = handler_lock.lock().await;

    if handler.queue().len() > 0 {
      let current = match handler.queue().current() {
        Some(t) => t,
        None => return text_response(ctx, command, "Nothing to skip".to_string()).await
      };

      match handler.queue().skip() {
        Err(e) => {
          error!("Error skipping track: {}", e);
          text_response(ctx, command, "Nothing to skip".to_string()).await
        },
        Ok(_) => {
          let title = current
            .metadata()
            .title
            .clone()
            .unwrap_or("N/A".to_string());

          let length = format_duration_live(
          current
              .metadata()
              .duration
              .unwrap_or(Duration::from_secs(0)),
            title.clone()
          ).0;

          match command
            .edit_original_interaction_response(&ctx.http, |response| {
              response
                .create_embed(|embed| {
                  embed
                    .title("Skipped")
                    .colour(EMBED_COLOUR)
                    .fields(vec![
                      ("Track", title, true),
                      ("Length", length, true),
                    ])
                })
            }).await {
              Ok(_m) => Ok(()),
              Err(e) => Err(e)
            }
        }
      }
    } else {
      text_response(ctx, command, "Nothing to skip".to_string()).await
    }
  }

  fn info(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
      .name("skip")
      .description("Skip the currently playing song")
  }

}