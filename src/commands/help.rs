use serenity::all::{CommandInteraction, Context, CreateCommand, CreateEmbed, CreateInteractionResponseMessage};

pub async fn run(
    ctx: &Context,
    interaction: &CommandInteraction,
) -> Result<(), serenity::Error> {
    
    let embed = CreateEmbed::new()
        .title("Help")
        .description("Commands available:")
        .field(
            "/ping",
            "Check if the bot is alive",
            false,
        )
        .field(
            "/leaderboard",
            "Get the leaderboard",
            false,
        )
        .field(
            "/calc",
            "Calculate your score",
            false,
        )
        .field(
            "/profile",
            "Get your profile",
            false,
        )
        .color(0xFF69B4);

    interaction
        .create_response(
            ctx,
            serenity::all::CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .embed(embed)
            ),
        )
        .await?;

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("help")
        .description("Get help with the bot")
}