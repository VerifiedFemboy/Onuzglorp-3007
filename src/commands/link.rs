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
    let user_id = interaction.user.id.get().to_string();

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

        match profile.discord_id {
            Some(discord_id) => {
                let collection = database.get_collection("onuzglorp-bot", "users");
                if discord_id == user_id {
                    //TODO: Logic to link the profile to the user
                } else {
                    interaction
                        .edit_response(
                            ctx,
                            EditInteractionResponse::new()
                                .content("⚠️ Your discord account **doesn't match** with the TUF profile.\nPlease make sure you linked the correct discord account on TUForums."),
                        )
                        .await
                        .expect("Failed to edit response");
                }
            }
            None => {
                interaction
                    .edit_response(
                        ctx,
                        EditInteractionResponse::new()
                            .content("⚠️ This profile hasn't linked to any Discord account"),
                    )
                    .await
                    .expect("Failed to edit response");
            }
        }
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
