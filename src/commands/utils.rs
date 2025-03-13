use crate::constants::EMBED_COLOUR;
use serenity::Error;
use serenity::builder::CreateEmbed;
use serenity::builder::EditInteractionResponse;
use serenity::client::Context;
use serenity::model::application::CommandInteraction;

pub fn remove_md_characters<S>(s: S) -> String
where
    S: ToString,
{
    s.to_string()
        .replace('_', r"\_")
        .replace('*', r"\*")
        .replace('~', r"\~")
        .replace('`', r"\`")
        .replace('>', r"\>")
        .replace('<', r"\<")
        .replace('[', r"\[")
        .replace(']', r"\]")
}

pub async fn text_response<D>(
    ctx: &Context,
    command: &CommandInteraction,
    text: D,
) -> Result<(), Error>
where
    std::string::String: From<D>,
{
    match command
        .edit_response(
            &ctx.http,
            EditInteractionResponse::new()
                .embed(CreateEmbed::new().title(text).colour(EMBED_COLOUR)),
        )
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
