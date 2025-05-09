pub async fn get_leaderboard(
    offset: u32,
    length: usize,
) -> Result<Vec<(u64, String, f64, f64, u64)>, Box<dyn std::error::Error + Send + Sync>> {
    let response = reqwest::get(format!(
        "https://api.tuforums.com/v2/database/leaderboard?query=&sortBy=rankedScore&order=desc&offset={}&limit={}&showBanned=hide",
        offset, length
    ))
    .await
    .expect("Failed to send request");

    if !response.status().is_success() {
        return Err(format!("Request failed with status: {}", response.status()).into());
    }

    let json = response
        .json::<serde_json::Value>()
        .await
        .expect("Failed to parse JSON");

    let mut leaderboard = Vec::new();

    let results = json["results"]
        .as_array()
        .ok_or("Missing or invalid 'results' field in JSON")?
        .iter()
        .take(length)
        .cloned()
        .collect::<Vec<_>>();

    for entry in results {
        let ranked_score = entry["rankedScore"].as_f64().unwrap_or(0.0);
        let avg_accuracy = entry["averageXacc"].as_f64().unwrap_or(0.0) * 100.0;
        let position = entry["rankedScoreRank"].as_u64().unwrap_or(0);
        let player = &entry["player"];
        // let playeruser = &player["user"];
        let username = player["name"].as_str().unwrap_or("Unknown").to_string();
        let id = player["id"].as_u64().unwrap_or(0);
        leaderboard.push((position, username, ranked_score, avg_accuracy, id));
    }

    Ok(leaderboard)
}
