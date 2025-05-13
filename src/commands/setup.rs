use std::{time::Duration, vec};

use serenity::all::{
    CommandInteraction, Context, CreateCommand, CreateInteractionResponse,
    CreateInteractionResponseMessage, CreateSelectMenu, CreateSelectMenuKind,
    CreateSelectMenuOption, EditInteractionResponse, Permissions,
};

use crate::database::Database;

pub async fn run(
    ctx: &Context,
    interaction: &CommandInteraction,
    database: &Database,
) -> Result<(), Box<dyn std::error::Error>> {
    let guild_id = interaction.guild_id.unwrap();
    let user_id = interaction.user.id;

    let collection = database
        .get_collection("onuzglorp-bot", "setups")
        .await
        .expect("Failed to get collection");

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
        Some(interaction) => {
            if interaction.data.custom_id == "setup-menu" {
                let selected_option = &interaction.data.custom_id;

                match selected_option.as_str() {
                    "rand-lvl-channel" => {
                        println!("Selected option: Daily Random LVL channel");
                    }
                    _ => {
                        interaction
                            .edit_response(
                                ctx,
                                EditInteractionResponse::new()
                                    .content("❌ Invalid option selected")
                                    .components(vec![]),
                            )
                            .await
                            .unwrap();
                        return Ok(());
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

pub fn register() -> CreateCommand {
    CreateCommand::new("setup")
        .description("Setup the bot")
        .default_member_permissions(Permissions::ADMINISTRATOR)
        .dm_permission(false)
}
