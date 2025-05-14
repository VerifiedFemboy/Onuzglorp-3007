use serenity::all::{
    CommandInteraction, Context, CreateCommand, CreateEmbed, CreateEmbedAuthor,
    CreateInteractionResponse, CreateInteractionResponseMessage, EditInteractionResponse,
};

use crate::{
    tuforums::level::{Level, get_level, get_total_levels},
    utils::get_video_id,
};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    interaction
        .create_response(
            ctx,
            CreateInteractionResponse::Defer(
                CreateInteractionResponseMessage::new().content("Fetching random level..."),
            ),
        )
        .await?;

    let total_levels = match get_total_levels().await {
        Ok(levels) => levels,
        Err(_) => {
            interaction
                .edit_response(
                    ctx,
                    EditInteractionResponse::new().content("Failed to fetch levels."),
                )
                .await?;
            return Ok(());
        }
    };

    let random_level = rand::random::<u64>() % total_levels;

    let level = match get_level(random_level as u32).await {
        Ok(level) => level,
        Err(_) => {
            interaction
                .edit_response(
                    ctx,
                    EditInteractionResponse::new().content(format!(
                        "Unable to retrieve the level with ID {}. Please try again later.",
                        random_level
                    )),
                )
                .await?;
            return Ok(());
        }
    };

    interaction
        .edit_response(
            ctx,
            EditInteractionResponse::new().embed(level_embed(level)),
        )
        .await?;

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("random_lvl").description("Get a random level")
}

pub fn level_embed(level: Level) -> CreateEmbed {
    let embed = CreateEmbed::new()
        .author(
            CreateEmbedAuthor::new(format!(
                "{} - {} | ID: {}",
                level.artist, level.title, level.id
            ))
            .icon_url(level.difficulty.icon)
            .url(format!("https://tuforums.com/levels/{}", level.id)),
        )
        .field(
            "**First Clear**",
            format!("``{}``", level.first_clear),
            true,
        )
        .field("**Total Clears**", level.clears.to_string(), true)
        .field(
            "**Highest Accuracy**",
            format!("{:.2}%", level.highest_acc),
            true,
        )
        .field("**Highest Score**", "soon", true)
        .field("**Highest Speed**", "soon", true)
        .field("**Download**", format!("[[link]]({})", level.dl_link), true)
        .image(format!(
            "https://i.ytimg.com/vi/{}/maxresdefault.jpg",
            get_video_id(&level.vido_link)
        ))
        .color(level.difficulty.color);
    embed
}
