use super::difficulty::{Difficulty, convert_from_hex_to_rgb};

pub struct Level {
    pub id: u32,
    pub title: String,
    pub artist: String,
    pub creator: String,
    pub difficulty: Difficulty,
    pub score_base: f64,
}

pub async fn get_level(id: u32) -> Result<Level, Box<dyn std::error::Error + Sync + Send>> {
    let resposne = reqwest::get(format!(
        "https://api.tuforums.com/v2/database/levels/{id}?includeRatings=true"
    ))
    .await
    .expect("Failed to send request");

    let json: serde_json::Value = resposne.json().await.expect("Failed to parse JSON");

    if !json["error"].is_null() {
        return Err("Level not found".into());
    }

    let map = &json["level"];
    let diff = &map["difficulty"];

    let beatmap = Level {
        id,
        title: map["song"]
            .as_str()
            .expect("Failed to get song value from JSON")
            .to_string(),
        artist: map["artist"]
            .as_str()
            .expect("Failed to get artist value from JSON")
            .to_string(),
        creator: map["creator"]
            .as_str()
            .expect("Failed to get song value from JSON")
            .to_string(),
        difficulty: Difficulty {
            name: "".to_string(),
            icon: diff["icon"].as_str().unwrap().to_string(),
            color: convert_from_hex_to_rgb(diff["color"].as_str().unwrap()),
            score_base: diff["baseScore"].as_f64().unwrap(),
        },
        score_base: map["baseScore"].as_f64().unwrap_or(0.),
    };

    Ok(beatmap)
}
