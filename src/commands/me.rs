use crate::{commands::Command, constants::EMBED_COLOUR, handlers::utils::text_response};
use serenity::{
    Error, async_trait,
    builder::{CreateCommand, CreateEmbed, CreateEmbedFooter, EditInteractionResponse},
    client::Context,
    model::application::CommandInteraction,
};

pub struct Me;

#[async_trait]
impl Command for Me {
    async fn execute(ctx: &Context, command: &CommandInteraction) -> Result<(), Error> {
        let avatar = ctx.cache.current_user().avatar_url();
        if let Some(avatar) = avatar {
            match command
                .edit_response(
                    &ctx.http,
                    EditInteractionResponse::new().embed(
                        CreateEmbed::new()
                            .colour(EMBED_COLOUR)
                            .image(avatar)
                            .footer(CreateEmbedFooter::new("💩")),
                    ),
                )
                .await
            {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        } else {
            text_response(ctx, command, "🍊").await
        }
    }

    const NAME: &'static str = "me";

    fn info() -> CreateCommand {
        CreateCommand::new(Self::NAME).description("🍊")
    }
}
