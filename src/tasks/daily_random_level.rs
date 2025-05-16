use std::time::Duration;

use serenity::{
    all::{Channel, ChannelId, Context, CreateMessage},
    futures::TryStreamExt,
};
use tokio::time::sleep;

use crate::{
    LogLevel,
    commands::random_lvl::level_embed,
    database::Database,
    log_message,
    tuforums::level::{get_level, request_random_lvl_id},
};
use chrono::Duration as ChronoDuration;

pub async fn run_task(ctx: &Context, database: &Database) {
    log_message("Launching daily random map task", LogLevel::Info);

    let collection = database
        .get_collection("onuzglorp-bot", "setups")
        .await
        .expect("Failed to get collection");

    let ctx = ctx.clone();

    tokio::spawn(async move {
        loop {
            let now = chrono::Utc::now();
            let next_midnight = (now + ChronoDuration::days(1))
                .date_naive()
                .and_hms_opt(0, 0, 0)
                .unwrap();
            let duration_until_midnight = (next_midnight - now.naive_utc()).to_std().unwrap();

            println!("Sleeping for {:?}", duration_until_midnight);
            sleep(duration_until_midnight).await;

            let filter = mongodb::bson::doc! { "type": "daily-random-lvl-channel" };
            let cursor = collection
                .find(filter)
                .await
                .expect("Failed to find documents");
            let results: Vec<_> = cursor
                .try_collect()
                .await
                .expect("Failed to collect documents");

            let level_id = loop {
                // Loop until we get a valid level ID
                match request_random_lvl_id().await {
                    Ok(id) => {
                        break id;
                    }
                    Err(_) => {
                        println!("Failed to fetch random level ID.");
                    }
                };
            };
            let level = match get_level(level_id).await {
                Ok(level) => level,
                Err(_) => {
                    println!("Unable to retrieve the level with ID {}.", level_id);
                    continue;
                }
            };

            let level_embed = level_embed(level);
            let message = CreateMessage::new().embed(level_embed);

            for doc in results {
                let channel_id = doc.get_str("channel_id").unwrap();
                let channel_id = ChannelId::new(channel_id.parse::<u64>().unwrap());

                if let Ok(channel) = channel_id.to_channel(&ctx).await {
                    if let Channel::Guild(g_channel) = channel {
                        sleep(Duration::from_secs(1)).await; // Sleep for 1 second to avoid rate limits
                        if let Err(e) = g_channel.send_message(&ctx.http, message.clone()).await {
                            eprintln!("Failed to send embed: {:?}", e);
                        }
                    }
                }
            }
        }
    });
}
