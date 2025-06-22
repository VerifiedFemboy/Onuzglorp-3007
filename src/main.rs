use std::{sync::Arc, vec};

use crate::{cache_manager::CacheManager, tasks::clear_cache};
use commands::leaderboard::LeaderboardHandler;
use database::Database;
use dotenv::dotenv;
use serenity::{
    Client,
    all::{
        ActivityData, Command, Context, CreateInteractionResponse,
        CreateInteractionResponseMessage, EventHandler, GatewayIntents, Interaction, Ready,
    },
    async_trait,
};
use tasks::{change_status, daily_random_level};
use tokio::sync::Mutex;

mod anilist;
mod cache_manager;
mod commands;
mod database;
mod formulas;
mod logger;
mod tasks;
mod tuforums;
mod utils;

struct Handler {
    database: Database,
    cache_manager: Arc<Mutex<CacheManager>>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let content = match command.data.name.as_str() {
                "ping" => {
                    commands::ping::run(&ctx, &command).await.unwrap();
                    None
                }
                "leaderboard" => {
                    commands::leaderboard::run(&ctx, &command).await.unwrap();
                    None
                }
                "calcscore" => {
                    commands::calc_score::run(&ctx, &command).await.unwrap();
                    None
                }
                "calcacc" => {
                    commands::calc_acc::run(&ctx, &command).await.unwrap();
                    None
                }
                "profile" => {
                    commands::profile::run(&ctx, &command, &self.database, &self.cache_manager)
                        .await
                        .unwrap();
                    None
                }
                "clear" => {
                    commands::clear::run(&ctx, &command).await.unwrap();
                    None
                }
                "help" => {
                    commands::help::run(&ctx, &command).await.unwrap();
                    None
                }
                "random_lvl" => {
                    commands::random_lvl::run(&ctx, &command).await.unwrap();
                    None
                }
                "link" => {
                    commands::link::run(&ctx, &command, &self.database)
                        .await
                        .unwrap();
                    None
                }
                "setup" => {
                    commands::setup::run(&ctx, &command, &self.database)
                        .await
                        .unwrap();
                    None
                }
                "cache" => {
                    commands::cache_info::run(&ctx, &command, &self.cache_manager)
                        .await
                        .unwrap();
                    None
                }
                /* "anilist" => {
                    commands::anilist::run(&ctx, &command, &self.database, &self.cache_manager)
                        .await
                        .unwrap();
                    None
                } */
                _ => Some("Unknown command".to_string()),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);

                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {:?}", why);
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        let commands = Command::set_global_commands(
            &ctx.http,
            vec![
                commands::help::register(),
                commands::calc_score::register(),
                commands::calc_acc::register(),
                commands::leaderboard::register(),
                commands::ping::register(),
                commands::profile::register(),
                commands::clear::register(),
                commands::random_lvl::register(),
                commands::link::register(),
                commands::setup::register(),
                commands::cache_info::register(),
                // commands::anilist::register(),
            ],
        )
        .await;

        if let Err(why) = commands {
            println!("Failed to register commands: {:?}", why);
        } else {
            println!("{} is connected!", ready.user.name);
        }

        daily_random_level::run_task(&ctx, &self.database).await;
        change_status::run_task(&ctx).await;
        // actix_web_main::run_task(&self.cache_manager)
        //     .await
        //     .expect("Failed to start Actix web server");
        clear_cache::run_task(&self.cache_manager).await;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let args: Vec<String> = std::env::args().collect();
    let token_env = if args.contains(&"dev".to_string()) {
        dotenv::var("DISCORD_TEST_TOKEN").expect("Expected a test token in the environment")
    } else {
        dotenv::var("DISCORD_TOKEN").expect("Expected a token in the environment")
    };

    let mongo_uri = dotenv::var("MONGO_URI").expect("Expected a mongo uri in the environment");

    let database = database::connect(&mongo_uri)
        .await
        .expect("Failed to connect to the database");

    let cache_manager = Arc::new(Mutex::new(CacheManager::new(database.clone())));

    let mut client = Client::builder(token_env, GatewayIntents::all())
        .event_handler(Handler {
            database,
            cache_manager,
        })
        .event_handler(LeaderboardHandler)
        .activity(ActivityData::watching("TUForums"))
        .await?;

    match client.start().await {
        Ok(()) => println!("Client started successfully"),
        Err(why) => println!("Client error: {:?}", why),
    }

    Ok(())
}

mod tests {
    #[cfg(test)]
    mod anilist_tests {
        use crate::anilist::anilist_user::get_anilist_user_info;

        #[tokio::test]
        async fn test_anilist_request() {
            let username = "VerifiedFemboy"; // Replace with a valid Anilist username
            get_anilist_user_info(username).await;
        }
    }
}
