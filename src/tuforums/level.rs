use super::difficulty::{Difficulty, convert_from_hex_to_rgb};

pub struct Level {
    pub id: u32,
    pub title: String,
    pub artist: String,
    pub creator: String,
    pub difficulty: Difficulty,
    pub score_base: f64,
    pub clears: u64,
    pub highest_acc: f64,
    pub first_clear: String,
    pub dl_link: String,
    pub vido_link: String,
}

pub async fn get_level(id: u32) -> Result<Level, Box<dyn std::error::Error + Sync + Send>> {
    let resposne = reqwest::get(format!("https://api.tuforums.com/v2/database/levels/{id}"))
        .await
        .expect("Failed to send request");

    let json: serde_json::Value = resposne.json().await.expect("Failed to parse JSON");

    if !json["error"].is_null() {
        return Err("Level not found".into());
    }

    let map = &json["level"];
    let diff = &map["difficulty"];
    let is_cleared = &map["isCleared"].as_bool().unwrap_or(false);

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
        clears: map["clears"].as_i64().unwrap_or(0) as u64,
        highest_acc: map["highestAccuracy"].as_f64().unwrap_or(0.) * 100.,
        first_clear: if *is_cleared {
            format!(
                "{} | {}",
                map["firstPass"]["player"]["name"].as_str().unwrap_or("-"),
                chrono::DateTime::parse_from_rfc3339(
                    map["firstPass"]["vidUploadTime"]
                        .as_str()
                        .unwrap_or("unknown")
                )
                .map(|dt| dt.format("%b %d, %Y").to_string()) // Use %b for abbreviated month
                .unwrap_or_else(|_| "unknown".to_string())
            )
        } else {
            "-".to_string()
        },
        dl_link: map["dlLink"].as_str().unwrap_or("unknown").to_string(),
        vido_link: map["videoLink"].as_str().unwrap_or("unknown").to_string(),
    };

    Ok(beatmap)
}

#[allow(dead_code)]
pub async fn get_total_levels() -> Result<u64, Box<dyn std::error::Error + Sync + Send>> {
    let resposne = reqwest::get("https://api.tuforums.com/v2/database/statistics")
        .await
        .expect("Failed to send request");

    let json: serde_json::Value = resposne.json().await.expect("Failed to parse JSON");

    if !json["error"].is_null() {
        return Err("Could not fetch stats".into());
    }

    let total_levels = json["overview"]["totalLevels"].as_u64().unwrap_or(0);

    Ok(total_levels)
}

pub async fn request_random_lvl_id() -> Result<u32, Box<dyn std::error::Error + Send + Sync>> {
    let response = match reqwest::get("https://api.tuforums.com/v2/database/levels?limit=1&offset=0&query=&sort=RANDOM_ASC&deletedFilter=hide&clearedFilter=show&pguRange=P1,U20&specialDifficulties=Marathon,Gimmick")
    .await {
        Ok(res) => res,
        Err(_) => {
            eprintln!("Failed to fetch random level ID");
            return Err("Failed to fetch random level ID".into());
        }
    };

    let json: serde_json::Value = response.json().await.expect("Failed to parse JSON");

    let level_id = match json["results"][0]["id"].as_u64() {
        Some(id) => id,
        None => {
            eprintln!("Failed to get level ID from response ID: {}", json["results"]["id"]);
            return Err("Failed to get level ID".into());
        }
    };

    Ok(level_id as u32)
}
