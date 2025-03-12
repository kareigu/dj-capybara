use crate::commands::utils::text_response;
use serenity::Error;
use serenity::async_trait;
use serenity::builder::CreateCommand;
use serenity::model::application::CommandInteraction;
use serenity::prelude::Context;

mod join;
pub use join::Join;
mod leave;
pub use leave::Leave;
mod play;
pub use play::Play;
mod skip;
pub use skip::Skip;
mod queue;
pub use queue::Queue;
mod seek;
pub use seek::Seek;
mod status;
pub use status::Status;
mod stop;
pub use stop::Stop;
mod capybara;
pub use capybara::Capybara;
mod me;
pub use me::Me;
mod info;
pub use info::Info;
mod eval;
pub use eval::Eval;
mod pause;
pub use pause::Pause;
mod resume;
pub use resume::Resume;

#[async_trait]
pub trait Command {
  async fn execute(ctx: &Context, command: &CommandInteraction) -> Result<(), Error>;
  fn info() -> CreateCommand;
  const NAME: &'static str;
}

pub fn command_list() -> Vec<CreateCommand> {
  vec![
    Join::info(),
    Leave::info(),
    Play::info(),
    Capybara::info(),
    Seek::info(),
    Skip::info(),
    Queue::info(),
    Me::info(),
    Info::info(),
    Stop::info(),
    Eval::info(),
    Pause::info(),
    Resume::info(),
    Status::info(),
  ]
}

pub async fn execute(name: &str, ctx: &Context, command: &CommandInteraction) -> Result<(), Error> {
  match name {
    _ if name == Join::NAME => Join::execute(ctx, command),
    _ if name == Leave::NAME => Leave::execute(ctx, command),
    _ if name == Play::NAME => Play::execute(ctx, command),
    _ if name == Seek::NAME => Seek::execute(ctx, command),
    _ if name == Skip::NAME => Skip::execute(ctx, command),
    _ if name == Queue::NAME => Queue::execute(ctx, command),
    _ if name == Stop::NAME => Stop::execute(ctx, command),
    _ if name == Capybara::NAME => Capybara::execute(ctx, command),
    _ if name == Me::NAME => Me::execute(ctx, command),
    _ if name == Info::NAME => Info::execute(ctx, command),
    _ if name == Eval::NAME => Eval::execute(ctx, command),
    _ if name == Pause::NAME => Pause::execute(ctx, command),
    _ if name == Resume::NAME => Resume::execute(ctx, command),
    _ if name == Status::NAME => Status::execute(ctx, command),
    _ => Box::pin(text_response(ctx, command, "Invalid command")),
  }
  .await
}
