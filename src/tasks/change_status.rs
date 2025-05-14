use std::time::Duration;

use serenity::all::{ActivityData, Context};
use tokio::{spawn, time::sleep};

pub async fn run_task(ctx: &Context) {
    let ctx = ctx.clone();

    let activities = vec![
        ActivityData::playing("ADOFAI"),
        ActivityData::listening("the community"),
        ActivityData::watching("TUForums"),
        ActivityData::competing("the leaderboard"),
        ActivityData::custom("onuz-globulus"),
        ActivityData::custom("U727"),
        ActivityData::custom("inferno ex ex"),
        ActivityData::custom("polska gurom"),
        ActivityData::custom("when you see it"),
        ActivityData::custom("heart rate: 727 bpm 🔥"),
        ActivityData::custom("developed by emi_neko"),
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
