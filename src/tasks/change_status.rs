use std::time::Duration;

use serenity::all::{ActivityData, Context};
use tokio::{spawn, time::sleep};

use crate::info;

pub async fn run_task(ctx: &Context) {
    info!("Launching change status task");
    let ctx = ctx.clone();

    let activities = vec![
        ActivityData::playing("ADOFAI"),
        ActivityData::listening("the community"),
        ActivityData::watching("TUForums"),
        ActivityData::competing("the leaderboard"),
        ActivityData::custom("Big Thanks to BypassedChicken for hosting! 💕"),
        ActivityData::custom("developed by emilia_pusheen"),
    ];

    spawn(async move {
        loop {
            for activity in &activities {
                ctx.set_activity(Some(activity.clone()));
                sleep(Duration::from_secs(15)).await;
            }
        }
    });
}
