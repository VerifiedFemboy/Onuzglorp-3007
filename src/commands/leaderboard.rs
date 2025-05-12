use serenity::{
    all::{
        ButtonStyle, CommandInteraction, CommandOptionType, Context, CreateActionRow, CreateButton,
        CreateCommand, CreateCommandOption, CreateEmbed, CreateEmbedFooter,
        CreateInteractionResponse, CreateInteractionResponseMessage, EditInteractionResponse,
        EventHandler, Interaction,
    },
    async_trait,
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

    let embed = embedos(leaders, page);

    let lb_prev = CreateButton::new(format!("lb_previous:{}", page - 1))
        .label("⬅️")
        .style(ButtonStyle::Primary)
        .disabled(true);

    let lb_next = CreateButton::new(format!("lb_next:{}", page + 1))
        .label("➡️")
        .style(ButtonStyle::Primary)
        .disabled(false);

    interaction
        .edit_response(
            ctx,
            EditInteractionResponse::new()
                .embed(embed)
                .components(vec![CreateActionRow::Buttons(vec![lb_prev, lb_next])]),
        )
        .await?;

    Ok(())
}

pub struct LeaderboardHandler;

#[async_trait]
impl EventHandler for LeaderboardHandler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Component(component) = interaction {
            let custom_id = component.data.custom_id.as_str();

            match custom_id.split(':').collect::<Vec<_>>().as_slice() {
                ["lb_previous", page_str] | ["lb_next", page_str] => {
                    if let Ok(page) = page_str.parse::<u32>() {
                        let offset = (page - 1) * 15;
                        let leaders = get_leaderboard(offset, 15).await.unwrap_or_else(|_| vec![]);

                        let embed = embedos(leaders, page);

                        let lb_prev = CreateButton::new(format!("lb_previous:{}", page - 1))
                            .label("⬅️")
                            .style(ButtonStyle::Primary)
                            .disabled(page <= 1);

                        let lb_next = CreateButton::new(format!("lb_next:{}", page + 1))
                            .label("➡️")
                            .style(ButtonStyle::Primary);

                        component
                            .create_response(
                                &ctx.http,
                                CreateInteractionResponse::UpdateMessage(
                                    CreateInteractionResponseMessage::new()
                                        .embed(embed)
                                        .components(vec![CreateActionRow::Buttons(vec![
                                            lb_prev, lb_next,
                                        ])]),
                                ),
                            )
                            .await
                            .expect("Failed to update leaderboard page");
                    }
                }
                _ => {
                    println!("Invalid button format");
                }
            }
        }
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("leaderboard")
        .description("Get the leaderboard from the TUForums")
        .dm_permission(true)
        .add_option(
            CreateCommandOption::new(CommandOptionType::Integer, "page", "Page number")
                .min_int_value(1)
                .required(false),
        )
}

fn embedos(leaders: Vec<(u64, String, f64, f64, u64)>, page: u32) -> CreateEmbed {
    CreateEmbed::new()
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
        .footer(CreateEmbedFooter::new(format!("page {page}")))
}
