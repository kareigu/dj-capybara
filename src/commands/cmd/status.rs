use crate::{commands::cmd::Command, constants};
use constants::EMBED_COLOUR;
use serenity::{
  Error, async_trait,
  builder::{CreateCommand, CreateEmbed, EditInteractionResponse},
  client::Context,
  model::application::CommandInteraction,
};
use tracing::error;

pub struct Status;

#[async_trait]
impl Command for Status {
  async fn execute(ctx: &Context, command: &CommandInteraction) -> Result<(), Error> {
    let yt_dlp_version = get_runtime_info("yt-dlp", ["--version"]);
    let uname = get_runtime_info("uname", ["-or"]);
    let uptime = get_runtime_info("uptime", []);

    let ping = {
      let start_time = std::time::Instant::now();
      _ = ctx.http.get_gateway().await;
      let end_time = std::time::Instant::now();
      format!("{}ms", end_time.duration_since(start_time).as_millis(),)
    };

    command
      .edit_response(
        &ctx.http,
        EditInteractionResponse::new().embed(
          CreateEmbed::new()
            .colour(EMBED_COLOUR)
            .title("Status")
            .fields([
              ("Version", constants::PACKAGE_VERSION, true),
              ("Rust", constants::RUST_VERSION, true),
              ("LLVM", constants::LLVM_VERSION, true),
              ("yt-dlp", &yt_dlp_version, true),
              ("Commit", constants::GIT_DESC, true),
              ("Ping", &ping, true),
              ("uname", &uname, false),
              ("Host", constants::HOST_TRIPLE, false),
              ("Uptime", &uptime, false),
              ("Build", constants::BUILD_TIMESTAMP, false),
            ]),
        ),
      )
      .await?;

    Ok(())
  }

  const NAME: &'static str = "status";

  fn info() -> CreateCommand {
    CreateCommand::new(Self::NAME).description("display capybara status")
  }
}

fn get_runtime_info<I>(command: &str, args: I) -> String
where
  I: IntoIterator<Item = &'static str>,
{
  match std::process::Command::new(command).args(args).output() {
    Ok(o) => String::from_utf8(o.stdout).unwrap_or_else(|_| String::from("N/A")),
    Err(e) => {
      error!("failed getting yt-dlp version: {}", e);
      String::from("N/A")
    }
  }
}
