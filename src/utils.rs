use serenity::all::CommandInteraction;
use sys_info;

//TODO: find a possible way to make those functions to one as a generic function
#[allow(dead_code)]
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

pub struct MemoryInfo {
    pub total: String,
    pub used: String,
    pub free: String,
    pub swap_total: String,
    pub swap_free: String,
}

pub fn get_memory_info() -> MemoryInfo {
    fn format_bytes(kb: u64) -> String {
        const KB: u64 = 1024;
        const MB: u64 = KB * 1024;
        const GB: u64 = MB * 1024;

        match kb {
            b if b >= GB => format!("{:.2} GB", b as f64 / GB as f64),
            b if b >= MB => format!("{:.2} MB", b as f64 / MB as f64),
            b if b >= KB => format!("{:.2} KB", b as f64 / KB as f64),
            b => format!("{} B", b),
        }
    }

    let mem = sys_info::mem_info().unwrap_or_else(|_| sys_info::MemInfo {
        total: 0,
        free: 0,
        avail: 0,
        buffers: 0,
        cached: 0,
        swap_total: 0,
        swap_free: 0,
    });

    let used = mem.total.saturating_sub(mem.free);

    MemoryInfo {
        total: format_bytes(mem.total * 1024),
        used: format_bytes(used * 1024),
        free: format_bytes(mem.free * 1024),
        swap_total: format_bytes(mem.swap_total * 1024),
        swap_free: format_bytes(mem.swap_free * 1024),
    }
}