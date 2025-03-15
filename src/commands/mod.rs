use crate::handlers::utils::text_response;
use serenity::{
    Error, async_trait, builder::CreateCommand, model::application::CommandInteraction,
    prelude::Context,
};

mod join;
use join::*;
mod leave;
use leave::*;
mod play;
use play::*;
mod skip;
use skip::*;
mod queue;
use queue::*;
mod seek;
use seek::*;
mod status;
use status::*;
mod stop;
use stop::*;
mod capybara;
use capybara::*;
mod me;
use me::*;
mod info;
use info::*;
mod eval;
use eval::*;
mod pause;
use pause::*;
mod resume;
use resume::*;

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
