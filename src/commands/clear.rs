use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage,
    EditInteractionResponse,
};

use crate::tuforums::clear_info::get_clear_info;

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    let id = interaction.data.options[0].value.as_i64().unwrap_or(0) as u64;

    interaction
        .create_response(
            ctx,
            CreateInteractionResponse::Defer(
                CreateInteractionResponseMessage::new().content("Getting the clear..."),
            ),
        )
        .await
        .unwrap();

    match get_clear_info(id).await {
        Ok(clear) => {
            let embed = CreateEmbed::new()
                .title(format!("Clear info | ID: {}", &id))
                .field("Player", clear.player_name, true)
                .field("Ranked score", format!("{:.2}", clear.ranked_score), true)
                .field("", "", false)
                .field("Feeling Rating", clear.feeling_rating, true)
                .field("Score", format!("{:.2}", clear.score), true)
                .field("", "", false)
                .field("Accuracy", format!("{:.2}", clear.accuracy), true)
                .field("Vido", format!("[vido_link]({})", clear.video_link), true)
                .field("", "", false)
                .field(
                    "Judgements",
                    format!(
                        "```ansi
[2;31m{}[0m [2;33m{}[0m [2;32m{}[0m [1;32m{}[0m [2;32m{}[0m [2;33m{}[0m [2;31m{}[0m```",
                        clear.judgements.0,
                        clear.judgements.1,
                        clear.judgements.2,
                        clear.judgements.3,
                        clear.judgements.4,
                        clear.judgements.5,
                        clear.judgements.6
                    ),
                    false,
                )
                .url(clear.video_link);

            interaction
                .edit_response(ctx, EditInteractionResponse::new().embed(embed))
                .await
                .unwrap();
        }
        Err(e) => {
            interaction
                .edit_response(
                    ctx,
                    EditInteractionResponse::new().content(format!("Error: {}", e)),
                )
                .await
                .unwrap();
            return Ok(());
        }
    }

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("clear")
        .description("Get info of a clear")
        .default_member_permissions(serenity::all::Permissions::MANAGE_MESSAGES)
        .add_option(
            CreateCommandOption::new(CommandOptionType::Integer, "clear_id", "Clear ID")
                .required(true),
        )
}
