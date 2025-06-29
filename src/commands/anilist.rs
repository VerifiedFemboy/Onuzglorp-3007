use std::error::Error;

use mongodb::{
    Collection,
    bson::{Document, doc},
};
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateInteractionResponse, CreateInteractionResponseMessage, EditInteractionResponse,
};

use crate::{
    anilist::anilist_user::{get_anilist_by_id, get_anilist_by_name, send_anilist_profile},
    database::Database,
};

pub async fn run(
    ctx: &Context,
    interaction: &CommandInteraction,
    database: &Database,
) -> Result<(), serenity::Error> {
    let user_id = interaction.user.id.get() as i64;
    let options = &interaction.data.options;

    let collection = if let Some(collection) = database
        .get_collection("onuzglorp-bot", "linked_anilist")
        .await
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

    if options.is_empty() {
        interaction
            .create_response(
                ctx,
                CreateInteractionResponse::Defer(
                    CreateInteractionResponseMessage::new().content("Fetching AniList user..."),
                ),
            )
            .await?;

        let user = collection
            .find_one(doc! { "_id": user_id })
            .await
            .expect("Failed to find user in database");

        if let None = user {
            interaction
                .edit_response(
                    ctx,
                    EditInteractionResponse::new()
                        .content("⚠️ You haven't linked Anilist profile yet!"),
                )
                .await
                .expect("Failed to edit the reposne");
            return Ok(());
        }

        let profile_id = user.unwrap().get_i64("anilist_id").ok().unwrap();

        let anilist_data = if let Some(data) = get_anilist_by_id(profile_id).await {
            data
        } else {
            interaction
                .edit_response(
                    ctx,
                    EditInteractionResponse::new().content("⚠️ AniList user not found!"),
                )
                .await
                .expect("Failed to edit the response");
            return Ok(());
        };

        send_anilist_profile(ctx, interaction, anilist_data).await;
    } else {
        if options.len() > 1 {
            interaction
                .create_response(
                    ctx,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new()
                            .content("⚠️ You cannot use more arguments than one!"),
                    ),
                )
                .await
                .expect("Failed to send the response");
            return Ok(());
        }

        interaction
            .create_response(
                ctx,
                CreateInteractionResponse::Defer(CreateInteractionResponseMessage::new()),
            )
            .await?;

        match options[0].name.as_str() {
            "search_by_username" => {}
            "link" => {
                match link_profile(
                    user_id,
                    options[0].value.as_str().expect("Failed to parse"),
                    collection,
                )
                .await
                {
                    Ok(res) => {
                        interaction
                            .edit_response(ctx, EditInteractionResponse::new().content(res))
                            .await
                            .expect("Failed to edit the response");
                        return Ok(());
                    }
                    Err(e) => {
                        interaction
                            .edit_response(
                                ctx,
                                EditInteractionResponse::new().content(format!("Error! {}", e)),
                            )
                            .await
                            .expect("Failed to edit the response");
                        return Ok(());
                    }
                }
            }
            _ => {
                interaction
                    .edit_response(
                        ctx,
                        EditInteractionResponse::new().content("This argument doesn't exist"),
                    )
                    .await
                    .expect("Failed to edit the response");
            }
        }
    }

    Ok(())
}

pub fn register() -> CreateCommand {
    let options = vec![
        CreateCommandOption::new(
            CommandOptionType::String,
            "search_by_username",
            "The AniList username to search for",
        )
        .required(false),
        CreateCommandOption::new(
            CommandOptionType::String,
            "link",
            "A link to an AniList profile",
        )
        .min_int_value(1)
        .required(false),
    ];

    CreateCommand::new("anilist")
        .description("Get information from AniList")
        .set_options(options)
}

async fn link_profile(
    user_id: i64,
    profile_username: &str,
    collection: Collection<Document>,
) -> Result<String, Box<dyn Error + std::marker::Send + std::marker::Sync>> {
    let anilist_user = match get_anilist_by_name(profile_username).await {
        Some(data) => data,
        None => {
            return Err("AniList user not found".into());
        }
    };

    let doc = doc! {
        "_id": user_id,
        "anilist_id": anilist_user.id,
    };

    let insert_result = collection.insert_one(doc).await;

    match insert_result {
        Ok(_) => {
            return Ok(format!(
                "✅ Successfully linked! Anilist profile id: {}",
                anilist_user.id
            ));
        }
        Err(_) => return Err("Failed to insert the data".into()),
    }
}
