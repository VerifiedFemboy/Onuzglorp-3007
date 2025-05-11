use std::vec;

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

mod commands;
mod formulas;
mod tuforums;
mod utils;
mod database;
struct Handler;

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
                    commands::profile::run(&ctx, &command).await.unwrap();
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
            ],
        )
        .await;

        if let Err(why) = commands {
            println!("Failed to register commands: {:?}", why);
        } else {
            println!("{} is connected!", ready.user.name);
        }
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

    let database = Database::new(mongo_uri);

    let db = database.connect().await.expect("Failed to connect to the database");

    let mut client = Client::builder(token_env, GatewayIntents::all())
        .event_handler(Handler)
        .event_handler(LeaderboardHandler)
        .activity(ActivityData::watching("TUForums"))
        .await?;

    match client.start().await {
        Ok(()) => println!("Client started successfully"),
        Err(why) => println!("Client error: {:?}", why),
    }

    Ok(())
}
