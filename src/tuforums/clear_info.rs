pub struct ClearInfo {
    pub player_name: String,
    pub ranked_score: f64,
    pub feeling_rating: String,
    pub score: f64,
    pub accuracy: f64,
    pub video_link: String,
    pub judgements: Judgements,
}

pub struct Judgements(pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8);

pub async fn get_clear_info(
    id: u64,
) -> Result<ClearInfo, Box<dyn std::error::Error + Sync + Send>> {
    let response = reqwest::get(format!("https://api.tuforums.com/v2/database/passes/{id}"))
        .await
        .expect("Failed to send the request");

    let json: serde_json::Value = response.json().await.expect("Failed to parse JSON");

    let feeling_rating = json["feelingRating"].as_str().unwrap_or("none").to_string();

    let player_name = json["player"]["name"]
        .as_str()
        .unwrap_or("none")
        .to_string();
    let video_link = json["videoLink"].as_str().unwrap_or("none").to_string();

    let judgements = &json["judgements"];

    let judgements = Judgements(
        judgements["earlyDouble"].as_u64().unwrap_or(0) as u8,
        judgements["earlySingle"].as_u64().unwrap_or(0) as u8,
        judgements["ePerfect"].as_u64().unwrap_or(0) as u8,
        judgements["perfect"].as_u64().unwrap_or(0) as u8,
        judgements["lPerfect"].as_u64().unwrap_or(0) as u8,
        judgements["lateSingle"].as_u64().unwrap_or(0) as u8,
        judgements["lateDouble"].as_u64().unwrap_or(0) as u8,
    );

    let clear_info = ClearInfo {
        player_name,
        judgements,
        feeling_rating,
        accuracy: 0.,
        ranked_score: 0.,
        score: 0.,
        video_link,
    };

    Ok(clear_info)
}
