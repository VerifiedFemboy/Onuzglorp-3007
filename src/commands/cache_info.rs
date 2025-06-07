use std::sync::Arc;

use serenity::all::{Color, CommandInteraction, Context, CreateCommand, CreateEmbed, CreateInteractionResponseMessage};

use crate::{cache_manager::CacheManager, tuforums::profile::Profile, utils::get_memory_info};

pub async fn run(
    ctx: &Context,
    interaction: &CommandInteraction,
    cache: &Arc<tokio::sync::Mutex<CacheManager>>,
) -> Result<(), serenity::Error> {
    let cache = cache.lock().await;
    let memory_info = get_memory_info();

    let embed = CreateEmbed::new()
        .title("Cache Information")
        .field(
            "Cache Size",
            format!("**{}**", cache.cache.len()),
            true,
        )
        .field("", "", true)
        .field("Total Profiles in Cache", 
            format!("**{}**", cache.get_all_entries::<Profile>().len()),
            true,
        )
        .field("Memory Information",
            format!(
                "``Total Memory: {}`` ``Used Memory: {}`` \n``Free Memory: {}`` \n``Swap Total: {}`` ``Swap Free: {}``",
                memory_info.total,
                memory_info.used,
                memory_info.free,
                memory_info.swap_total,
                memory_info.swap_free
            ),
            true,
        )
        .color(Color::DARK_PURPLE);

    interaction
        .create_response(
            ctx,
            serenity::all::CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .embed(embed),
            ),
        )
        .await
        .expect("Something went wrong while creating response");

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("cache").description("Provides information about the current cache state.")
}
