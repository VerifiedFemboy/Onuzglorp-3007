use tokio::sync::Mutex;

use crate::cache_manager::{CacheManager, LiveTime};

use super::difficulty::{self, Difficulty, convert_from_hex_to_rgb};

#[derive(Debug, Clone)]
pub struct Profile {
    pub name: String,
    pub username: String,
    pub avatar: Option<String>,
    pub discord_id: Option<String>,
    pub stats: Stats,
}

#[derive(Debug, Clone)]
pub struct Stats {
    pub rank: Rank,
    pub general_score: f64,
    pub ranked_score: f64,
    pub avg_xacc: f64,
    pub top_diff: Difficulty,
}

#[derive(Debug, Clone)]
pub struct Rank(pub i64);

pub async fn get_profile(
    id: u64,
    cache_manager: Option<&Mutex<CacheManager>>,
) -> Result<(Profile, bool), Box<dyn std::error::Error + Send + Sync>> {
    if let Some(cache) = cache_manager {
        if let Some(profile) = cache
            .lock()
            .await
            .get::<Profile>(format!("profile_{id}").as_str())
            .await
        {
            return Ok((profile.clone(), true)); // TODO: come up with a better way without cloning to less use memory
        }
    }

    let response = reqwest::get(format!("https://api.tuforums.com/v2/database/players/{id}"))
        .await
        .expect("Failed to send request");

    let json: serde_json::Value = response.json().await.expect("Failed to parse JSON");

    let name = json["name"].as_str().unwrap_or("Unknown").to_string();

    let username = if let Some(usr_name) = json["discordUsername"].as_str() {
        if usr_name.is_empty() {
            "".to_string()
        } else {
            format!("({usr_name})")
        }
    } else {
        "".to_string()
    };

    let avatar = json["pfp"]
        .as_str()
        .filter(|s| !s.is_empty() && *s != "none")
        .map(|s| s.to_string());

    let discord_id = json["discordId"].as_str().map(|s| s.to_string());

    let difficulty = &json["topDiff"];

    let top_diff = difficulty::Difficulty {
        name: difficulty["name"].as_str().unwrap_or("Unknown").to_string(),
        icon: difficulty["icon"].as_str().unwrap_or("Unknown").to_string(),
        color: convert_from_hex_to_rgb(difficulty["color"].as_str().unwrap_or("#000000")),
        score_base: difficulty["baseScore"].as_f64().unwrap_or(0.),
    };

    let stats = Stats {
        rank: Rank(json["stats"]["rankedScoreRank"].as_i64().unwrap_or(0)),
        general_score: json["generalScore"].as_f64().unwrap_or(0.),
        ranked_score: json["rankedScore"].as_f64().unwrap_or(0.),
        avg_xacc: json["averageXacc"].as_f64().unwrap_or(0.),
        top_diff,
    };

    let profile = Profile {
        name,
        username,
        avatar,
        discord_id,
        stats,
    };

    if let Some(cache) = cache_manager {
        cache.lock().await.add(
            format!("profile_{id}"),
            profile.clone(),
            Some(LiveTime::Minutes(10)),
            None,
        );
    }

    // println!("{:#?}", profile);

    Ok((profile, false))
}
