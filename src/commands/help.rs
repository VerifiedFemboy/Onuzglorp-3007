use serenity::all::{
    CommandInteraction, Context, CreateCommand, CreateEmbed, CreateInteractionResponseMessage,
};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    let embed = CreateEmbed::new()
        .title("Help")
        .description("Commands available:")
        .field("/calcacc", "Calculate your accuracy", false)
        .field("/calcscore", "Calculate your score", false)
        .field("/clear", "Get clear info", false)
        .field("/leaderboard", "Get the leaderboard", false)
        .field("/ping", "Check if the bot is alive", false)
        .field("/profile", "Get your profile", false)
        .field("/randomlvl", "Get a random level", false)
        .field("/setup", "Setup the bot to your expectetions", false)
        .color(0xFF69B4);

    interaction
        .create_response(
            ctx,
            serenity::all::CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().embed(embed),
            ),
        )
        .await?;

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("help").description("Get help with the bot")
}
