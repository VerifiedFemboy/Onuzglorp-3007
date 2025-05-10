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
