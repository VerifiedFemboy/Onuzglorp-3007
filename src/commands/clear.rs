use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption, CreateEmbed, CreateEmbedAuthor, CreateInteractionResponse, CreateInteractionResponseMessage, EditInteractionResponse
};

use crate::{formulas::{acc_by_judgement, score_final}, tuforums::clear_info::{get_clear_info, Judgements}};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    let id = interaction.data.options[0].value.as_i64().unwrap_or(0) as u64;

    interaction
        .create_response(
            ctx,
            CreateInteractionResponse::Defer(
                CreateInteractionResponseMessage::new().content("Getting the clear..."),
            ),
        )
        .await
        .unwrap();

    match get_clear_info(&id).await {
        Ok(clear) => {
            let is_worlds_first = &clear.is_worlds_first;
            let is_16k = &clear.is_16k;
            let is_12k = &clear.is_12k;
            let is_no_miss = &clear.is_no_miss;
            let is_no_hold_tap = &clear.is_no_hold_tap;

            let beatmap = clear.beatmap;

            let mut embed = CreateEmbed::new()
                .author(
                    CreateEmbedAuthor::new(format!("{} - {} | ID: {}", beatmap.artist, beatmap.title, beatmap.id))
                    .icon_url(beatmap.difficulty.icon)
                )
                .title(format!("Clear info | ID: {}", id))
                .field("**Player**", clear.player_name, true)
                .field("**Feeling Rating**", clear.feeling_rating, true)
                .field("", "", false)
                .field("**Accuracy**", format!("{:.2}%", clear.accuracy), true)
                .field("**Score**", format!("{:.2}", clear.score), true)
                .field("**Speed**", format!("{}x", clear.speed), true)
                .field(format!("{}",
                if *is_16k {
                    "16K"
                } else if *is_12k {
                    "12K"
                } else {
                    ""
                }), "", true)
                .field(
                    if *is_no_hold_tap {
                        "**No Hold Tap"
                    } else {
                        ""
                    }
                    , "", false)
                .field(
                if *is_worlds_first {
                    "🏆 World's First"
                } else {
                    ""
                }, "", false)
                .field("", format!("
                ```ansi
[2;31m{}[0m [2;33m{}[0m [2;32m{}[0m [1;32m{}[0m [2;32m{}[0m [2;33m{}[0m [2;31m{}[0m```", clear.judgements.0,
                clear.judgements.1,
                clear.judgements.2,
                clear.judgements.3,
                clear.judgements.4,
                clear.judgements.5,
                clear.judgements.6,
                ), false)
                .image(format!("https://i.ytimg.com/vi/{}/maxresdefault.jpg", get_video_id(&clear.video_link)))
                .thumbnail(clear.player_avatar)
                .color(beatmap.difficulty.color);

                if !is_no_miss {
                    let base_score = if beatmap.score_base == 0. {
                        beatmap.difficulty.score_base
                    } else {
                        beatmap.score_base
                    };

                    let accuracy = acc_by_judgement(Judgements(0, clear.judgements.1, 
                        clear.judgements.2,
                        clear.judgements.3,
                        clear.judgements.4,
                        clear.judgements.5,
                        clear.judgements.6,
                        )) * 100.;

                        let score = score_final(base_score, accuracy, 1, 0, clear.speed);

                    embed = embed
                        .field("**If nomiss**", "", false)
                        .field("**Accuracy**", format!("{:.2}%", &accuracy), true)
                        .field("**Score**", format!("{:.2}", score), true);
                }

                embed = embed.field("", format!("[{}]({})",clear.video_title, clear.video_link), false);

            interaction
                .edit_response(ctx, EditInteractionResponse::new().embed(embed))
                .await
                .unwrap();
        }
        Err(e) => {
            interaction
                .edit_response(
                    ctx,
                    EditInteractionResponse::new().content(format!("Error: {}", e)),
                )
                .await
                .unwrap();
            return Ok(());
        }
    }

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("clear")
        .description("Get info of a clear")
        .default_member_permissions(serenity::all::Permissions::MANAGE_MESSAGES)
        .add_option(
            CreateCommandOption::new(CommandOptionType::Integer, "clear_id", "Clear ID")
                .required(true),
        )
}


fn get_video_id(vido_link: &str) -> String {
    let mut video_id = String::new();
    if let Some(start) = vido_link.find("v=") {
        video_id = vido_link[start + 2..].to_string();
    } else if let Some(start) = vido_link.find("youtu.be/") {
        video_id = vido_link[start + 9..].to_string();
    } else if let Some(start) = vido_link.find("/embed/") {
        video_id = vido_link[start + 7..].to_string();
    }
    if let Some(end) = video_id.find('&') {
        video_id.truncate(end);
    }
    video_id
    
}