use std::vec;

use serenity::all::{
    Color, CommandInteraction, Context, CreateCommand, CreateCommandOption, CreateEmbed,
    CreateEmbedFooter, CreateInteractionResponseMessage, EditInteractionResponse,
};

use crate::{formulas::score_final, tuforums::level::get_level};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    let start_time = std::time::Instant::now();

    interaction
        .create_response(
            ctx,
            serenity::all::CreateInteractionResponse::Defer(
                CreateInteractionResponseMessage::new().content("Getting a beatmap..."),
            ),
        )
        .await
        .expect("Something went wrong while creating response");

    let id = interaction
        .data
        .options
        .iter()
        .find(|option| option.name == "level_id")
        .and_then(|option| option.value.as_i64())
        .unwrap_or(0) as u32;

    let x_acc = interaction
        .data
        .options
        .iter()
        .find(|option| option.name == "x_acc")
        .and_then(|option| option.value.as_f64())
        .unwrap_or(0.);

    let misses = interaction
        .data
        .options
        .iter()
        .find(|option| option.name == "misses")
        .and_then(|option| option.value.as_i64())
        .unwrap_or(0) as u32;

    let tile_count = interaction
        .data
        .options
        .iter()
        .find(|option| option.name == "tile_count")
        .and_then(|option| option.value.as_i64())
        .unwrap_or(0) as u32;

    let speed = interaction
        .data
        .options
        .iter()
        .find(|option| option.name == "speed")
        .and_then(|option| option.value.as_f64())
        .unwrap_or(1.);

    let ranked_position = interaction
        .data
        .options
        .iter()
        .find(|option| option.name == "ranked_position")
        .and_then(|option| option.value.as_i64())
        .unwrap_or(1) as f64;

    let level = match get_level(id).await {
        Ok(map) => map,
        Err(e) => {
            interaction
                .edit_response(
                    ctx,
                    EditInteractionResponse::new().content(format!(
                        "Failed to get the level with id: {}.\n**Error: {}**",
                        id, e
                    )),
                )
                .await
                .expect("Failed to edit the response");
            return Ok(());
        }
    };

    let base_score = if level.score_base == 0. {
        level.difficulty.score_base
    } else {
        level.score_base
    };

    let score = score_final(base_score, x_acc, tile_count, misses, speed);
    let ranked_score = if ranked_position >= 20. {
        0.0
    } else {
        score * 0.9f64.powf(ranked_position - 1.)
    };

    let stop_time = std::time::Instant::now();
    let elapsed_time = stop_time.duration_since(start_time);

    let embed = CreateEmbed::new()
    .title(format!(
        "{} - {} | ID: {}",
        level.artist, level.title, level.id
    ))
    .description(format!("*charted by {}*", level.creator))
    .field("Using those informations", format!("``base score: {}`` | ``xAcc: {}``\n``tile Count: {}`` | ``misses: {}`` | ``speed: {}x``", base_score, x_acc, tile_count, misses, speed), false)
    .thumbnail(level.difficulty.icon.to_string())
    .field("Your score", format!("**{:.2}**", score), true)
    .field("Your ranked score", format!("**{:.2} (#{})**", ranked_score, ranked_position), true)
    .color(Color::from_rgb(level.difficulty.color.0, level.difficulty.color.1, level.difficulty.color.2))
    .footer(CreateEmbedFooter::new(format!("Response time {} ms", elapsed_time.as_millis().to_string())));

    interaction
        .edit_response(ctx, EditInteractionResponse::new().add_embed(embed))
        .await
        .expect("Failed to edit the response");

    Ok(())
}

pub fn register() -> CreateCommand {
    let id = CreateCommandOption::new(
        serenity::all::CommandOptionType::Integer,
        "level_id",
        "The id for level you want to calculate",
    )
    .required(true);

    let x_acc = CreateCommandOption::new(
        serenity::all::CommandOptionType::Number,
        "x_acc",
        "X-Accuracy",
    )
    .min_int_value(0)
    .max_int_value(100)
    .required(true);

    let misses = CreateCommandOption::new(
        serenity::all::CommandOptionType::Integer,
        "misses",
        "Your misses",
    )
    .min_int_value(0)
    .required(true);

    let tile_count = CreateCommandOption::new(
        serenity::all::CommandOptionType::Integer,
        "tile_count",
        "Your tile count",
    )
    .min_int_value(0)
    .required(true);

    let speed = CreateCommandOption::new(
        serenity::all::CommandOptionType::Number,
        "speed",
        "Your speed",
    )
    .required(false);

    let ranked_position = CreateCommandOption::new(
        serenity::all::CommandOptionType::Integer,
        "ranked_position",
        "Your ranked position",
    )
    .min_int_value(0)
    .required(false);

    CreateCommand::new("calcscore")
        .description("Calculate your score")
        .dm_permission(true)
        .default_member_permissions(serenity::all::Permissions::empty())
        .set_options(vec![id, x_acc, misses, tile_count, speed, ranked_position])
}
