use serenity::all::{
    CommandInteraction, Context, CreateCommand, CreateEmbed, CreateEmbedFooter,
    CreateInteractionResponseMessage, EditInteractionResponse,
};

use crate::{formulas, tuforums::clear_info::Judgements, utils::get_option_as_string};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    let start_time = std::time::Instant::now();

    interaction
        .create_response(
            ctx,
            serenity::all::CreateInteractionResponse::Defer(
                CreateInteractionResponseMessage::new().content("Calculating accuracy..."),
            ),
        )
        .await
        .expect("Something went wrong while creating response");

    let judgements = get_option_as_string(interaction, "judgements", "0 0 0 0 0 0 0")
        .split(' ')
        .map(|s| s.parse::<u64>().unwrap_or(0))
        .collect::<Vec<u64>>();

    if judgements.len() != 7 {
        interaction
            .edit_response(
                ctx,
                EditInteractionResponse::new().content("Invalid judgements"),
            )
            .await
            .expect("Something went wrong while editing response");
        return Ok(());
    }

    let accuracy = formulas::acc_by_judgement(Judgements(
        judgements[0],
        judgements[1],
        judgements[2],
        judgements[3],
        judgements[4],
        judgements[5],
        judgements[6],
    )) * 100.0;

    let stop_time = std::time::Instant::now();
    let elapsed_time = stop_time.duration_since(start_time);

    let embed = CreateEmbed::new()
        .title("**Accuracy Calculation**")
        .field("Using those judgements", format!("
        ```ansi
[2;31m{}[0m [2;33m{}[0m [2;32m{}[0m [1;32m{}[0m [2;32m{}[0m [2;33m{}[0m [2;31m{}[0m```", 
        judgements[0],
        judgements[1],
        judgements[2],
        judgements[3],
        judgements[4],
        judgements[5],
        judgements[6],
        ),
        false)
        .field("**Your Accuracy is**", format!("{:.2}%", accuracy), false)
        .footer(CreateEmbedFooter::new(format!(
            "Calculated in {}ms",
            elapsed_time.as_millis()
        )))
        .color(0xFF69B4);

    interaction
        .edit_response(ctx, EditInteractionResponse::new().embed(embed))
        .await
        .expect("Something went wrong while editing response");

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("calcacc")
        .description("Calculate the accuracy from the judgements")
        .add_option(
            serenity::all::CreateCommandOption::new(
                serenity::all::CommandOptionType::String,
                "judgements",
                "The judgements to calculate the accuracy from",
            )
            .required(true),
        )
}
