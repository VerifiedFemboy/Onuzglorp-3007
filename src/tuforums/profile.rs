use super::difficulty::{self, Difficulty, convert_from_hex_to_rgb};

#[derive(Debug)]
pub struct Profile {
    pub name: String,
    pub username: String,
    pub avatar: String,
    pub discord_id: Option<String>,
    pub stats: Stats,
}

#[derive(Debug)]
pub struct Stats {
    pub rank: Rank,
    pub general_score: f64,
    pub ranked_score: f64,
    pub avg_xacc: f64,
    pub top_diff: Difficulty,
}

#[derive(Debug)]
pub struct Rank(pub i64);

pub async fn get_profile(id: u64) -> Result<Profile, Box<dyn std::error::Error + Send + Sync>> {
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

    let avatar = json["pfp"].as_str().unwrap_or("").to_string();

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

    Ok(profile)
}
