use std::sync::Arc;

use mongodb::bson::doc;
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption, CreateEmbed, CreateEmbedFooter, CreateInteractionResponse, CreateInteractionResponseMessage, EditInteractionResponse
};
use tokio::sync::Mutex;

use crate::{cache_manager::CacheManager, database::Database, log_message, tuforums::profile::get_profile, LogLevel};

pub async fn run(
    ctx: &Context,
    interaction: &CommandInteraction,
    database: &Database,
    cache_manager: &Arc<Mutex<CacheManager>>,
) -> Result<(), serenity::Error> {
    let start_time = std::time::Instant::now();

    let id = interaction
        .data
        .options
        .get(0)
        .and_then(|option| option.value.as_i64())
        .unwrap_or(0) as u64;

    let id = if id == 0 {
        let discord_id = interaction.user.id.get() as i64;
        match get_profile_linked(discord_id, database).await {
            Ok(profile_id) => profile_id,
            Err(_) => {
                interaction
                    .create_response(
                        ctx,
                        CreateInteractionResponse::Message(
                            CreateInteractionResponseMessage::new()
                                .content("❓ Could not find a linked profile.\nPlease link your profile using `/link`."),
                        ),
                    )
                    .await
                    .unwrap();
                return Ok(());
            }
        }
    } else {
        id
    };

    interaction
        .create_response(
            ctx,
            CreateInteractionResponse::Defer(
                CreateInteractionResponseMessage::new().content("Fetching profile..."),
            ),
        )
        .await
        .unwrap();

    match get_profile(id, Some(&cache_manager)).await {
        Ok(result) => {
            let profile = result.0;
            let cached = result.1;
            let stop_time = std::time::Instant::now();
            let elapsed_time = stop_time.duration_since(start_time);

            let embed = CreateEmbed::new()
                .title(format!("Profile of {} {}", profile.name, profile.username))
                .thumbnail(profile.avatar)
                .field(
                    "Rank",
                    {
                        let rank_str = profile.stats.rank.0.to_string();
                        format!(
                            "**#{}**",
                            if profile.stats.rank.0 == 1 {
                                "1 👑"
                            } else {
                                &rank_str
                            }
                        )
                    },
                    true,
                )
                .field(
                    "Ranked Score",
                    format!("**{:.2}**", profile.stats.ranked_score),
                    true,
                )
                .field(
                    "General Score",
                    format!("**{:.2}**", profile.stats.general_score),
                    true,
                )
                .field(
                    "Top Diff",
                    format!("**{}**", profile.stats.top_diff.name),
                    true,
                )
                .field(
                    "AVG X-Accuracy",
                    format!("**{:.2}%**", profile.stats.avg_xacc * 100.),
                    true,
                )
                .field(
                    "Discord",
                    format!(
                        "**{}**",
                        if let Some(id) = profile.discord_id {
                            format!("<@{}>", id)
                        } else {
                            "Not linked".to_string()
                        }
                    ),
                    true,
                )
                .color(profile.stats.top_diff.color)
                .footer(CreateEmbedFooter::new(format!(
                    "Response time: {} ms | Cache used: {}",
                    elapsed_time.as_millis(),
                    if cached { "Yes" } else { "No" }
                )));

            interaction
                .edit_response(ctx, EditInteractionResponse::new().embed(embed))
                .await
                .unwrap();
        }
        Err(e) => {
            interaction
                .edit_response(
                    ctx,
                    EditInteractionResponse::new()
                        .content(format!("Error fetching profile: {}", e)),
                )
                .await
                .unwrap();
            log_message(format!("Couldn't fetch profile {e}").as_str(), LogLevel::Error);
            return Ok(());
        }
    };

    Ok(())
}

async fn get_profile_linked(
    discord_id: i64,
    database: &Database,
) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
    let collection = database
        .get_collection("onuzglorp-bot", "users")
        .await
        .unwrap();

    let filter = doc! { "_id": discord_id };
    let user = collection.find_one(filter).await?;

    if let Some(user) = user {
        if let Some(profile_id) = user.get_i64("profile_id").ok() {
            return Ok(profile_id as u64);
        }
    }

    Err("User not found".into())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("profile")
        .description("Get a user's profile")
        .add_option(CreateCommandOption::new(
            CommandOptionType::Integer,
            "player_id",
            "Player ID",
        ))
}
