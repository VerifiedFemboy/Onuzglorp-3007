use crate::error;

const QUERY: &str = "query ($name: String) {
  User(name: $name) {
    id
    name
    avatar {
      large
      medium
    }
    about
    statistics {
      anime {
        count
        episodesWatched
        genres {
          genre
          minutesWatched
        }
        statuses {
          status
          mediaIds
        }
      }
    }
    favourites {
      anime {
        nodes {
          id
          title {
            english
          }
          bannerImage
          coverImage {
            large
            medium
            color
          }
          episodes
          genres
          description
          format
        }
      }
    }
  }
}
";

#[derive(Debug)]
pub struct AnilistUser {
    pub id: i32,
    pub name: String,
    pub avatar: Avatar,
    pub about: Option<String>,
}

#[derive(Debug)]
pub struct Avatar {
    pub large: String,
    pub medium: String,
}

pub async fn get_anilist_user_info(username: &str) -> Option<AnilistUser> {
    let client = reqwest::Client::new();
    let url = "https://graphql.anilist.co";

    let query = QUERY;
    let variables = serde_json::json!({
        "name": username,
    });

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "query": query,
            "variables": variables,
        }))
        .send()
        .await;

    match response {
        Ok(res) => {
            if res.status().is_success() {
                match res.json::<serde_json::Value>().await {
                    Ok(data) => {
                        if let Some(user) = data["data"]["User"].as_object() {
                            let id = user["id"].as_i64().unwrap_or(0) as i32;
                            let name = user["name"].as_str().unwrap_or("").to_string();
                            let avatar_large =
                                user["avatar"]["large"].as_str().unwrap_or("").to_string();
                            let avatar_medium =
                                user["avatar"]["medium"].as_str().unwrap_or("").to_string();
                            let about = user["about"].as_str().map(|s| s.to_string());

                            let anilist_user = AnilistUser {
                                id,
                                name,
                                avatar: Avatar {
                                    large: avatar_large,
                                    medium: avatar_medium,
                                },
                                about,
                            };
                            Some(anilist_user)
                        } else {
                            error!("User not found or invalid response format");
                            None
                        }
                    }
                    Err(e) => {
                        error!(format!("Failed to parse JSON response: {}", e));
                        None
                    }
                }
            } else {
                error!(format!("Error: {}", res.status()));
                None
            }
        }
        Err(e) => {
            error!(format!("Request failed: {}", e));
            None
        }
    }
}
