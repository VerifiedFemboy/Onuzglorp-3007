use std::{time::Duration, vec};

use mongodb::{
    Collection,
    bson::{Document, doc},
};
use serenity::all::{
    ChannelType, CommandInteraction, ComponentInteraction, ComponentInteractionDataKind, Context,
    CreateActionRow, CreateCommand, CreateInteractionResponse, CreateInteractionResponseMessage,
    CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption, EditInteractionResponse,
    Permissions,
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

    let channels = ctx.http.get_channels(guild_id).await.unwrap();

    let channel_options: Vec<CreateSelectMenuOption> = channels
        .iter()
        .filter(|c| c.kind == ChannelType::Text)
        .map(|channel| {
            CreateSelectMenuOption::new(format!("# {}", &channel.name), &channel.id.to_string())
                .description(format!("id: {}", &channel.id))
        })
        .collect();

    let channels_menu = CreateActionRow::SelectMenu(CreateSelectMenu::new(
        "select-channels",
        CreateSelectMenuKind::String {
            options: channel_options,
        },
    ));

    interaction
        .create_response(
            ctx,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content("Select the channel where the daily random level will be posted")
                    .components(vec![channels_menu]),
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
            if comp_interaction.data.custom_id == "select-channels" {
                let selected = match &comp_interaction.data.kind {
                    ComponentInteractionDataKind::StringSelect { values } => &values[0],
                    _ => panic!("unexpected interaction data kind"),
                };

                let channel_id = selected;

                if collection.find_one(doc! {"guild_id": guild_id.get().to_string(), "type": "daily-random-lvl-channel"}).await.unwrap().is_some() {
                    interaction
                        .edit_response(
                            ctx,
                            EditInteractionResponse::new()
                                .content("❌ Daily random level channel already setup")
                                .components(vec![]),
                        )
                        .await
                        .unwrap();
                    return;
                }

                collection
                    .insert_one(doc! {
                        "guild_id": guild_id.get().to_string(),
                        "channel_id": channel_id,
                        "type": "daily-random-lvl-channel",
                        "setup_by": user_id.to_string(),
                    })
                    .await
                    .expect("Failed to insert document");

                interaction
                    .edit_response(
                        ctx,
                        EditInteractionResponse::new()
                            .content("✅ Daily random level channel setup successfully")
                            .components(vec![]),
                    )
                    .await
                    .unwrap();
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
}

pub fn register() -> CreateCommand {
    CreateCommand::new("setup")
        .description("Setup the bot")
        .default_member_permissions(Permissions::ADMINISTRATOR)
        .dm_permission(false)
}
