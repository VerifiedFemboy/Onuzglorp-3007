use mongodb::bson::doc;
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateInteractionResponse, CreateInteractionResponseMessage, EditInteractionResponse,
};

use crate::{database::Database, tuforums::profile::get_profile};

pub async fn run(
    ctx: &Context,
    interaction: &CommandInteraction,
    database: &Database,
) -> Result<(), Box<dyn std::error::Error>> {
    let db = &database.client;

    let profile_id = interaction.data.options[0].value.as_i64().unwrap_or(0);

    let user_id = interaction.user.id.get() as i64;

    interaction
        .create_response(
            ctx,
            CreateInteractionResponse::Defer(
                CreateInteractionResponseMessage::new()
                    .content("Linking your profile...")
                    .ephemeral(true),
            ),
        )
        .await?;

    if let Some(_) = db {
        let profile = match get_profile(profile_id as u64).await {
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
                let collection = if let Some(collection) =
                    database.get_collection("onuzglorp-bot", "users").await
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

                if discord_id.parse::<i64>().unwrap_or_default() == user_id {
                    link_profile(
                        ctx,
                        interaction,
                        &collection,
                        doc! {
                            "_id": user_id,
                            "profile_id": profile_id as i64,
                        },
                    )
                    .await
                    .expect("Failed to link profile");
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

pub async fn link_profile(
    ctx: &Context,
    interaction: &CommandInteraction,
    collection: &mongodb::Collection<mongodb::bson::Document>,
    doc: mongodb::bson::Document,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if collection
        .find_one(doc! { "_id": doc.get_i64("_id")? })
        .await?
        .is_some()
    {
        interaction
            .edit_response(
                ctx,
                EditInteractionResponse::new().content("⚠️ Your profile is already linked"),
            )
            .await?;
        return Ok(());
    }

    if collection.insert_one(doc).await.is_ok() {
        interaction
            .edit_response(
                ctx,
                EditInteractionResponse::new()
                    .content("✅ Your profile has been linked successfully!"),
            )
            .await?;
    } else {
        interaction
            .edit_response(
                ctx,
                EditInteractionResponse::new().content("⚠️ Failed to link your profile"),
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
