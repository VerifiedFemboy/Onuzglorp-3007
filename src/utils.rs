use serenity::all::CommandInteraction;

//TODO: find a possible way to make those functions to one as a generic function

pub fn get_option_as_f64(interaction: &CommandInteraction, name: &str, default: f64) -> f64 {
    interaction
        .data
        .options
        .iter()
        .find(|option| option.name == name)
        .and_then(|option| option.value.as_f64())
        .unwrap_or(default)
}

pub fn get_option_as_string(interaction: &CommandInteraction, name: &str, default: &str) -> String {
    interaction
        .data
        .options
        .iter()
        .find(|option| option.name == name)
        .and_then(|option| option.value.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| default.to_string())
}

pub fn get_video_id(vido_link: &str) -> String {
    let mut video_id = String::new();
    if let Some(start) = vido_link.find("v=") {
        video_id = vido_link[start + 2..].to_string();
    } else if let Some(start) = vido_link.find("youtu.be/") {
        video_id = vido_link[start + 9..].to_string();
    } else if let Some(start) = vido_link.find("/embed/") {
        video_id = vido_link[start + 7..].to_string();
    }
    if let Some(end) = video_id.find('&') {
        video_id.truncate(end);
    }
    video_id
}
