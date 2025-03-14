use crate::{commands::cmd::Command, constants};
use constants::{EMBED_COLOUR, ShardLatencyKey};
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
        let uptime = {
            let (delta, start) = constants::uptime().await;
            let since = {
                let weeks = delta.num_weeks();
                let days = delta.num_days() - weeks * 7;
                let hours = delta.num_hours() - days * 24;
                let minutes = delta.num_minutes() - hours * 60;
                let seconds = delta.num_seconds() - minutes * 60;
                match (weeks, days, hours, minutes, seconds) {
                    (w, d, h, _, _) if w > 0 => format!("{w} weeks {d} days {h} hours"),
                    (_, d, h, m, _) if d > 0 => format!("{d} days {h} hours {m} minutes"),
                    (_, _, h, m, s) if h > 0 => format!("{h} hours {m} minutes {s} seconds"),
                    (_, _, _, m, s) if m > 0 => format!("{m} minutes {s} seconds"),
                    (_, _, _, _, s) => format!("{s} seconds"),
                }
            };
            format!("{} ({})", since, start.format("%Y-%m-%d %H:%M:%S"))
        };

        let ping = {
            let data = ctx.data.read().await;
            let latency_map = data
                .get::<ShardLatencyKey>()
                .expect("No latency_map in global data");
            let latency_map_lock = latency_map.read().await;

            latency_map_lock
                .get(&ctx.shard_id)
                .map_or(String::from("N/A"), |o| format!("{}ms", o.as_millis()))
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
