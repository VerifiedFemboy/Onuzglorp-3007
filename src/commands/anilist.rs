use std::sync::Arc;

use mongodb::bson::doc;
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateInteractionResponse, CreateInteractionResponseMessage, EditInteractionResponse,
};
use tokio::sync::Mutex;

use crate::{cache_manager::CacheManager, database::Database};

pub async fn run(
    ctx: &Context,
    interaction: &CommandInteraction,
    database: &Database,
    cache_manager: &Arc<Mutex<CacheManager>>,
) -> Result<(), serenity::Error> {
    let userId = interaction.user.id.get() as i64;
    let options = &interaction.data.options;

    if options.is_empty() {
        interaction
            .create_response(
                ctx,
                CreateInteractionResponse::Defer(
                    CreateInteractionResponseMessage::new().content("Fetching AniList user..."),
                ),
            )
            .await?;

        let collection = if let Some(collection) = database
            .get_collection("onuzglorp-bot", "linked_anilist")
            .await
        {
            collection
        } else {
            interaction
                .edit_response(
                    ctx,
                    EditInteractionResponse::new().content("⚠️ Database connection error"),
                )
                .await?;
            return Ok(());
        };
    } else {
        if options.len() > 1 {
            interaction
                .create_response(
                    ctx,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new()
                            .content("⚠️ You cannot use more arguments than one!"),
                    ),
                )
                .await
                .expect("Failed to send the response");
            return Ok(());
        }


    }

    Ok(())
}

pub fn register() -> CreateCommand {
    let options = vec![
        CreateCommandOption::new(
            CommandOptionType::String,
            "username",
            "The AniList username to search for",
        )
        .required(false),
        CreateCommandOption::new(
            CommandOptionType::String,
            "link",
            "A link to an AniList profile",
        )
        .min_int_value(1)
        .required(false),
    ];

    CreateCommand::new("anilist")
        .description("Get information from AniList")
        .set_options(options)
}
