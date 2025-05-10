use super::{beatmap::Beatmap, difficulty::{convert_from_hex_to_rgb, Difficulty}};

pub struct ClearInfo {
    pub is_worlds_first: bool,
    pub is_16k: bool,
    pub is_12k: bool,
    pub is_no_hold_tap: bool,
    pub player_name: String,
    pub player_avatar: String,
    pub feeling_rating: String,
    pub score: f64,
    pub accuracy: f64,
    pub speed: f64,
    pub video_title: String,
    pub video_link: String,
    pub judgements: Judgements,
    pub beatmap: Beatmap,
    pub is_no_miss: bool,
}

pub struct Judgements(pub u64, pub u64, pub u64, pub u64, pub u64, pub u64, pub u64);

pub async fn get_clear_info(
    id: &u64,
) -> Result<ClearInfo, Box<dyn std::error::Error + Sync + Send>> {
    let response = reqwest::get(format!("https://api.tuforums.com/v2/database/passes/{id}"))
        .await
        .expect("Failed to send the request");

    let json: serde_json::Value = response.json().await.expect("Failed to parse JSON");

    let is_worlds_first = json["isWorldsFirst"].as_bool().unwrap_or(false);

    let is_16k = json["is16K"].as_bool().unwrap_or(false);
    let is_12k = json["is12K"].as_bool().unwrap_or(false);
    let is_no_hold_tap = json["isNoHoldTap"].as_bool().unwrap_or(false);

    let score = json["scoreV2"].as_f64().unwrap_or(0.);

    let feeling_rating = json["feelingRating"].as_str().unwrap_or("none").to_string();

    let player_name = json["player"]["name"]
        .as_str()
        .unwrap_or("none")
        .to_string();

    let player_avatar = json["player"]["pfp"]
        .as_str()
        .unwrap_or("none")
        .to_string();


    let video_title = json["vidTitle"].as_str().unwrap_or("none").to_string();
    let video_link = json["videoLink"].as_str().unwrap_or("none").to_string();
    

    let judgements = &json["judgements"];

    let speed = json["speed"].as_f64().unwrap_or(0.).max(0.);

    let accuracy = judgements["accuracy"].as_f64().unwrap_or(0.) * 100.;
    let judgements = Judgements(
        judgements["earlyDouble"].as_u64().unwrap_or(0),
        judgements["earlySingle"].as_u64().unwrap_or(0),
        judgements["ePerfect"].as_u64().unwrap_or(0),
        judgements["perfect"].as_u64().unwrap_or(0),
        judgements["lPerfect"].as_u64().unwrap_or(0),
        judgements["lateSingle"].as_u64().unwrap_or(0),
        judgements["lateDouble"].as_u64().unwrap_or(0),
    );

    
    let is_no_miss = is_no_miss(&judgements);

    let beatmap = &json["level"];

    let beatmap = Beatmap {
        id: beatmap["id"].as_u64().unwrap_or(0) as u32,
        title: beatmap["song"].as_str().unwrap_or("none").to_string(),
        artist: beatmap["artist"].as_str().unwrap_or("none").to_string(),
        creator: beatmap["creator"].as_str().unwrap_or("none").to_string(),
        difficulty: Difficulty {
            name: "".to_string(),
            icon: beatmap["difficulty"]["icon"]
                .as_str()
                .unwrap_or("none")
                .to_string(),
            color: convert_from_hex_to_rgb(
                beatmap["difficulty"]["color"].as_str().unwrap_or("#000000"),
            ),
            score_base: beatmap["difficulty"]["baseScore"]
                .as_f64()
                .unwrap_or(0.)
                .max(0.),
        },
        score_base: beatmap["baseScore"].as_f64().unwrap_or(0.).max(0.),
    };

    let clear_info = ClearInfo {
        is_worlds_first,
        is_16k,
        is_12k,
        is_no_hold_tap,
        player_name,
        judgements,
        feeling_rating,
        accuracy,
        score,
        video_link,
        video_title,
        beatmap,
        speed,
        player_avatar,
        is_no_miss,
    };

    Ok(clear_info)
}


fn is_no_miss(judgement: &Judgements) -> bool {
    judgement.0 == 0
}