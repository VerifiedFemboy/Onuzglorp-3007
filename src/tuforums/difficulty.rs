#[derive(Debug, Clone)]
pub struct Difficulty {
    pub name: String,
    pub icon: String,
    pub color: (u8, u8, u8),
    pub score_base: f64,
}

pub fn convert_from_hex_to_rgb(hex: &str) -> (u8, u8, u8) {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
    (r, g, b)
}
