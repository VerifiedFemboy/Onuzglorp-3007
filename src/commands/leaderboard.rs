use serenity::all::{
    ButtonStyle, CommandInteraction, CommandOptionType, Context, CreateButton, CreateCommand,
    CreateCommandOption, CreateEmbed, CreateEmbedFooter, CreateInteractionResponse,
    CreateInteractionResponseMessage, EditInteractionResponse, Permissions,
};

use crate::tuforums::leaderboard::get_leaderboard;
// TODO: make buttons to change pages
pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    let page = interaction
        .data
        .options
        .get(0)
        .and_then(|option| option.value.as_i64())
        .unwrap_or(1) as u32;

    let offset = (page - 1) * 15;

    interaction
        .create_response(
            ctx,
            CreateInteractionResponse::Defer(
                CreateInteractionResponseMessage::new().content("Fetching leaderboard..."),
            ),
        )
        .await?;

    let leaders = match get_leaderboard(offset, 15).await {
        Ok(leaders) => leaders,
        Err(e) => {
            interaction
                .edit_response(
                    ctx,
                    EditInteractionResponse::new()
                        .content(format!("Error fetching leaderboard: {}", e)),
                )
                .await?;
            return Ok(());
        }
    };

    let embed = CreateEmbed::new()
        .title("Leaderboard")
        .field(
            "``username [id] | ranked score | avg accuracy``",
            format!(
                "{}",
                leaders
                    .iter()
                    .map(|(position, name, score, acc, id)| {
                        let crown = if *position == 1 { "👑 " } else { "" };
                        format!(
                            "``{:<3}. {:<20}[{}] | {:.2} | {:.2}%`` {crown}",
                            position, name, id, score, acc
                        )
                    })
                    .collect::<Vec<String>>()
                    .join("\n")
            ),
            false,
        )
        .color(0xFF69B4)
        .footer(CreateEmbedFooter::new(format!("Page {page} (buttons will be available soon)")));

    let lb_prev = CreateButton::new("lb_previous")
        .label("⬅️")
        .style(ButtonStyle::Primary)
        .disabled(true);

    let lb_next = CreateButton::new("lb_next")
        .label("➡️")
        .style(ButtonStyle::Primary)
        .disabled(true);

    interaction
        .edit_response(
            ctx,
            EditInteractionResponse::new()
                .embed(embed)
                .button(lb_prev)
                .button(lb_next),
        )
        .await?;
    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("leaderboard")
        .description("Get the leaderboard from the TUForums")
        .dm_permission(true)
        .default_member_permissions(Permissions::empty())
        .add_option(
            CreateCommandOption::new(CommandOptionType::Integer, "page", "Page number")
                .min_int_value(1)
                .required(false),
        )
}

// pub struct LeaderboardHandler;

// #[async_trait]
// impl EventHandler for LeaderboardHandler {
//     async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
//         match interaction {
//             Interaction::Component(component_interaction) => {
//                 match component_interaction.data.custom_id.as_str() {
//                     "lb_next" => {
//                         component_interaction.edit_response(
//                             ctx,
//                             EditInteractionResponse::new().content("balls")).await.unwrap();
//                     },
//                     _ => {}
//                 }
//             },
//             _ => {},
//         }
//     }
// }
