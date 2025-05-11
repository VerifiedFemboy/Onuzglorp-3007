use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage,
    EditInteractionResponse,
};

use crate::tuforums::profile::get_profile;

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    let id = interaction.data.options[0].value.as_i64().unwrap_or(0) as u64;
    interaction
        .create_response(
            ctx,
            CreateInteractionResponse::Defer(
                CreateInteractionResponseMessage::new().content("Fetching profile..."),
            ),
        )
        .await
        .unwrap();

    match get_profile(id).await {
        Ok(profile) => {
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
                    format!(":{}:", profile.stats.top_diff.name),
                    true,
                )
                .field(
                    "AVG X-Accuracy",
                    format!("**{:.2}%**", profile.stats.avg_xacc * 100.),
                    true,
                )
                .field("Discord", format!("**{}**", profile.discord_id), true)
                .color(profile.stats.top_diff.color);

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
            return Ok(());
        }
    };

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("profile")
        .description("Get a user's profile")
        .add_option(
            CreateCommandOption::new(CommandOptionType::Integer, "player_id", "Player ID")
                .required(true),
        )
}