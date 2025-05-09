use serenity::all::{
    CommandInteraction, Context, CreateCommand, CreateInteractionResponse,
    CreateInteractionResponseMessage,
};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    interaction
        .create_response(
            ctx,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content("Pong! 🏓")
                    .ephemeral(true),
            ),
        )
        .await?;
    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("ping")
        .description("Ping the bot")
        .dm_permission(true)
        .default_member_permissions(serenity::all::Permissions::empty())
}
