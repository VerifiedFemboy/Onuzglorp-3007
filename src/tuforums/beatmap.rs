use super::difficulty::{Difficulty, convert_from_hex_to_rgb};

pub struct Beatmap {
    pub id: u32,
    pub title: String,
    pub artist: String,
    pub creator: String,
    pub difficulty: Difficulty,
    pub score_base: f64,
}

pub async fn get_beatmap(id: u32) -> Result<Beatmap, Box<dyn std::error::Error + Sync + Send>> {
    let resposne = reqwest::get(format!(
        "https://api.tuforums.com/v2/database/levels/{id}?includeRatings=true"
    ))
    .await
    .expect("Failed to send request");

    let json: serde_json::Value = resposne.json().await.expect("Failed to parse JSON");

    let map = &json["level"];
    let diff = &map["difficulty"];

    let beatmap = Beatmap {
        id,
        title: map["song"].as_str().unwrap().to_string(),
        artist: map["artist"].as_str().unwrap().to_string(),
        creator: map["creator"].as_str().unwrap().to_string(),
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
