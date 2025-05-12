use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateInteractionResponse, CreateInteractionResponseMessage, EditInteractionResponse,
};

use crate::{database::Database, tuforums::profile, utils::get_option_as_f64};

pub async fn run(
    ctx: &Context,
    interaction: &CommandInteraction,
    database: &Database,
) -> Result<(), Box<dyn std::error::Error>> {
    let db = &database.client;
    let profile_id = get_option_as_f64(interaction, "profile_id", 0.) as u64;

    interaction
        .create_response(
            ctx,
            CreateInteractionResponse::Defer(
                CreateInteractionResponseMessage::new().content("Linking your profile..."),
            ),
        )
        .await?;

    if let Some(_) = db {
        let profile = match profile::get_profile(profile_id).await {
            Ok(profile) => profile,
            Err(_) => {
                interaction
                    .edit_response(
                        ctx,
                        EditInteractionResponse::new().content("Failed to fetch profile"),
                    )
                    .await?;
                return Ok(());
            }
        };

        let collection = database.get_collection("onuzglorp-bot", "users");
    } else {
        interaction
            .edit_response(
                ctx,
                EditInteractionResponse::new().content("Database connection error"),
            )
            .await?;
    }

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("link")
        .description("Link your profile of TUF")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "profile_id",
                "Your profile ID in TUF",
            )
            .required(true),
        )
}
