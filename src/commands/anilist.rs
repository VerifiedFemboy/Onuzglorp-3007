use std::sync::Arc;

use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
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
        .required(false),
    ];

    CreateCommand::new("anilist")
        .description("Get information from AniList")
        .set_options(options)
}
