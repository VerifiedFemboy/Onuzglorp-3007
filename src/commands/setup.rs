use std::{time::Duration, vec};

use mongodb::{
    Collection,
    bson::{Document, doc},
};
use serenity::all::{
    ChannelId, CommandInteraction, ComponentInteraction, ComponentInteractionDataKind, Context, CreateCommand, CreateInteractionResponse, CreateInteractionResponseMessage, CreateQuickModal, CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption, EditInteractionResponse, Permissions
};

use crate::database::Database;

pub async fn run(
    ctx: &Context,
    interaction: &CommandInteraction,
    database: &Database,
) -> Result<(), Box<dyn std::error::Error>> {
    let user_id = interaction.user.id;

    interaction
        .create_response(
            ctx,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content("What do you want to setup?")
                    .select_menu(CreateSelectMenu::new(
                        "setup-menu",
                        CreateSelectMenuKind::String {
                            options: vec![
                            CreateSelectMenuOption::new(
                                "Daily Random LVL channel",
                                "rand-lvl-channel",
                            )
                            .description(
                                "Select the channel where the daily random level will be posted",
                            ),
                        ],
                        },
                    )),
            ),
        )
        .await
        .unwrap();

    let response = interaction.get_response(ctx).await.unwrap();

    let collectors = response
        .await_component_interaction(&ctx.shard)
        .timeout(Duration::from_secs(60))
        .author_id(user_id)
        .await;

    match collectors {
        Some(comp_interaction) => {
            if comp_interaction.data.custom_id == "setup-menu" {
                let selected = match &comp_interaction.data.kind {
                    ComponentInteractionDataKind::StringSelect { values } => &values[0],
                    _ => panic!("unexpected interaction data kind"),
                };

                match selected.as_str() {
                    "rand-lvl-channel" => {
                        let collection = database
                            .get_collection("onuzglorp-bot", "setups")
                            .await
                            .expect("Failed to get collection");
                        let guild_id = comp_interaction.guild_id.unwrap();

                        if collection.find_one(doc! {"guild_id": &guild_id.get().to_string(), "type": "daily-random-lvl-channel"}).await.unwrap().is_some() {
                            interaction
                                .edit_response(
                                    ctx,
                                    EditInteractionResponse::new()
                                        .content("❌ Daily random level channel already setup")
                                        .components(vec![]),
                                )
                                .await
                                .unwrap();
                            return Ok(());
                        }

                        setup_random_lvl_channel(ctx, &comp_interaction, &collection).await;
                    }
                    _ => {
                        comp_interaction
                            .edit_response(
                                ctx,
                                EditInteractionResponse::new()
                                    .content("❌ Invalid selection")
                                    .components(vec![]),
                            )
                            .await
                            .unwrap();
                    }
                }
            }
        }
        None => {
            interaction
                .edit_response(
                    ctx,
                    EditInteractionResponse::new()
                        .content("❌ Setup timed out")
                        .components(vec![]),
                )
                .await
                .unwrap();
        }
    }

    Ok(())
}

pub async fn setup_random_lvl_channel(
    ctx: &Context,
    interaction: &ComponentInteraction,
    collection: &Collection<Document>,
) {
    let guild_id = interaction.guild_id.unwrap();
    let user_id = interaction.user.id;


    let response = interaction
        .quick_modal(ctx,
            CreateQuickModal::new("Text Channel Setup")
            .timeout(Duration::from_secs(60))
            .short_field("channel id")
        ).await.unwrap();


    match response {
        Some(modal_interaction) => {
                // Extract the channel id from the modal's fields
                let channel_input = modal_interaction.inputs[0].as_str();

                if channel_input.is_empty() {
                    modal_interaction
                        .interaction.edit_response(
                            ctx,
                            EditInteractionResponse::new()
                                .content("❌ Channel ID cannot be empty")
                                .components(vec![]),
                        )
                        .await
                        .unwrap();
                    return;
                }

                // Check if the provided channel ID exists in the guild
                let channel_id = match channel_input.parse::<u64>() {
                    Ok(id) => ChannelId::new(id),
                    Err(_) => {
                        modal_interaction.interaction
                            .create_response(
                                ctx,
                                CreateInteractionResponse::UpdateMessage(
                                    CreateInteractionResponseMessage::new()
                                        .content("❌ Invalid channel ID format")
                                        .components(vec![]),
                                )
                            )
                            .await
                            .unwrap();
                        return;
                    }
                };

                let channel = match ctx.http.get_channel(channel_id).await {
                    Ok(channel) => channel,
                    Err(_) => {
                        modal_interaction.interaction
                            .create_response(
                                ctx,
                                CreateInteractionResponse::UpdateMessage(
                                    CreateInteractionResponseMessage::new()
                                        .content("❌ Channel not found")
                                        .components(vec![]),
                                )
                            )
                            .await
                            .unwrap();
                        return;
                    }
                };

                if channel.guild().map(|g| g.guild_id) != Some(guild_id) {
                    modal_interaction.interaction
                        .create_response(
                            ctx,
                            CreateInteractionResponse::UpdateMessage(
                                CreateInteractionResponseMessage::new()
                                    .content("❌ Channel does not belong to this server")
                                    .components(vec![]),
                            )
                        )
                        .await
                        .unwrap();
                    return;
                }

                if collection.find_one(doc! {"guild_id": guild_id.get().to_string(), "type": "daily-random-lvl-channel"}).await.unwrap().is_some() {
                    modal_interaction.interaction
                        .create_response(
                            ctx,
                            CreateInteractionResponse::UpdateMessage(
                                CreateInteractionResponseMessage::new()
                                    .content("❌ Daily random level channel already setup")
                                    .components(vec![]),
                            ),
                        )
                        .await
                        .unwrap();
                    return;
                }

                collection
                    .insert_one(doc! {
                        "guild_id": guild_id.get().to_string(),
                        "channel_id": channel_input,
                        "type": "daily-random-lvl-channel",
                        "setup_by": user_id.to_string(),
                    })
                    .await
                    .expect("Failed to insert document");

                modal_interaction.interaction
                    .create_response(
                        ctx,
                        CreateInteractionResponse::UpdateMessage(
                            CreateInteractionResponseMessage::new()
                                .content(format!("✅ Daily random level channel setup in <#{}>", channel_input))
                                .components(vec![]),
                        ),
                    )
                    .await
                    .unwrap();

                let channel = ctx
                    .http
                    .get_channel(ChannelId::new(channel_input.parse::<u64>().unwrap()))
                    .await
                    .unwrap();

                channel
                    .guild()
                    .unwrap()
                    .say(
                        ctx,
                        "⚠️ **Random levels will be posted here every day at midnight UTC** ⚠️",
                    )
                    .await
                    .unwrap();
            
        }
        None => {
            interaction
                .create_response(
                    ctx,
                    CreateInteractionResponse::UpdateMessage(
                        CreateInteractionResponseMessage::new()
                            .content("❌ Setup timed out")
                            .components(vec![])
                    )
                )
                .await
                .unwrap();
        }
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("setup")
        .description("Setup the bot")
        .default_member_permissions(Permissions::ADMINISTRATOR)
        .dm_permission(false)
}
