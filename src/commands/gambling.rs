use std::sync::Arc;

use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use serenity::all::{
    Color, CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateEmbed, CreateEmbedFooter, CreateInteractionResponse, CreateInteractionResponseMessage,
    EditInteractionResponse,
};
use tokio::sync::Mutex;

use crate::{
    cache_manager::{CacheManager, LiveTime},
    database::Database,
    utils,
};

pub async fn run(
    ctx: &Context,
    interaction: &CommandInteraction,
    database: &Database,
    cache_manager: &Arc<Mutex<CacheManager>>,
) -> Result<(), serenity::Error> {
    let user_id = interaction.user.id;

    let mut is_new_acc = false;

    interaction
        .create_response(
            ctx,
            CreateInteractionResponse::Defer(
                CreateInteractionResponseMessage::new().content("Processing your gamble..."),
            ),
        )
        .await?;

    let (stats, cache_usage) = match get_gambling_stats(
        database,
        user_id.get() as i64,
        &mut is_new_acc,
        cache_manager,
    )
    .await
    {
        Ok(stats) => stats,
        Err(e) => {
            eprintln!("Error fetching gambling stats: {}", e);
            interaction
                .edit_response(
                    ctx,
                    EditInteractionResponse::new()
                        .content("An error occurred while fetching your gambling stats."),
                )
                .await?;
            return Ok(());
        }
    };

    let options = &interaction.data.options;

    if options.is_empty() {
        let embed_content = if is_new_acc {
            CreateEmbed::new()
                .description("Welcome to the gambling game! You have been given a starting amount of 1500 coins.")
        } else {
            CreateEmbed::new().description("You can gamble to try and increase your amount!")
        };

        let embed_content = embed_content
            .title("Your Gambling Stats")
            .field("**Amount**", stats.amount.to_string(), true)
            .field("**Wins**", stats.wins.to_string(), true)
            .field("**Losses**", stats.losses.to_string(), true)
            .field("", "", false)
            .field("**Win Streak**", stats.win_streak.to_string(), true)
            .field("**Loss Streak**", stats.loss_streak.to_string(), true)
            .field(
                "Last Win",
                stats
                    .last_win
                    .map(|t| utils::format_timestamp(t))
                    .unwrap_or_else(|| "Never".to_string()),
                true,
            )
            .field(
                "Last Loss",
                stats
                    .last_loss
                    .map(|t| utils::format_timestamp(t))
                    .unwrap_or_else(|| "Never".to_string()),
                true,
            )
            .footer(CreateEmbedFooter::new(format!(
                "Use /gambling <amount> to gamble! | Cache used: {}",
                if cache_usage { "Yes" } else { "No" }
            )))
            .color(Color::MEIBE_PINK)
            .thumbnail(&interaction.user.avatar_url().unwrap_or_default());

        interaction
            .edit_response(ctx, EditInteractionResponse::new().embed(embed_content))
            .await
            .expect("Failed to edit the response");
    } else {
        let amount = utils::get_option_as_f64(interaction, "amount", 0.0) as u64;
        
    }

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("gambling")
        .description("Risk your money for a chance to win more!")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "amount",
                "Amount of money to gamble",
            )
            .min_int_value(1),
        )
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GamblingStats {
    pub _id: i64, // User ID of the gambler
    pub amount: u64,
    pub wins: u64,
    pub losses: u64,
    pub win_streak: u64,
    pub loss_streak: u64,
    pub last_win: Option<u64>,  // Timestamp of the last win
    pub last_loss: Option<u64>, // Timestamp of the last loss
    pub cache_key: String,
}

async fn get_gambling_stats(
    database: &Database,
    user_id: i64,
    new_acc: &mut bool,
    cache_manager: &Arc<Mutex<CacheManager>>,
) -> Result<(GamblingStats, bool), mongodb::error::Error> {
    let mut cache = cache_manager.lock().await;

    if let Some(stats) = cache
        .get_serializable::<GamblingStats>(format!("gambling_stats_{}", user_id).as_str())
        .await
    {
        return Ok((stats.clone(), true));
    }

    let collection = database
        .get_collection_gen::<GamblingStats>("onuzglorp-bot", "gambling_stats")
        .await
        .expect("Failed to get gambling_stats collection");

    let filter = doc! { "_id": user_id };

    match collection.find_one(filter).await? {
        Some(stats) => {
            cache.add(
                format!("gambling_stats_{}", user_id),
                stats.clone(),
                Some(LiveTime::Minutes(1)),
                Some("gambling_stats".to_string()),
            );
            Ok((stats, false))
        }
        None => {
            let new_stats = GamblingStats {
                _id: user_id,
                amount: 1500, // Default starting amount
                wins: 0,
                losses: 0,
                win_streak: 0,
                loss_streak: 0,
                last_win: None,
                last_loss: None,
                cache_key: format!("gambling_stats_{}", user_id),
            };
            *new_acc = true; // Mark as new account
            collection.insert_one(new_stats.clone()).await?;
            cache.add(
                format!("gambling_stats_{}", user_id),
                new_stats.clone(),
                Some(LiveTime::Minutes(10)),
                Some("gambling_stats".to_string()),
            );
            Ok((new_stats, false))
        }
    }
}
