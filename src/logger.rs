use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;

const FILE_NAME: &str = "logs.txt";

pub fn get_local() -> String {
    let now = Local::now().format("%Y-%m-%d %I:%M:%S %p");
    now.to_string()
}

//TODO: Find out if it is really efficient 
pub fn write_log(level: &str, msg: &str) {
    let log_entry = format!("[{}] [{}] {}\n", get_local(), level, msg);
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(FILE_NAME)
        .unwrap();
    let _ = file.write_all(log_entry.as_bytes());
}

#[macro_export]
macro_rules! info {
    ($msg:expr) => {{
        println!("\x1b[32m[{}] [INFO] {}\x1b[0m", $crate::logger::get_local(), $msg);
        $crate::logger::write_log("INFO", &$msg.to_string());
    }};
}

#[macro_export]
macro_rules! warn {
    ($msg:expr) => {{
        println!("\x1b[33m[{}] [WARNING] {}\x1b[0m", $crate::logger::get_local(), $msg);
        $crate::logger::write_log("WARNING", &$msg.to_string());
    }};
}

#[macro_export]
macro_rules! error {
    ($msg:expr) => {{
        eprintln!("\x1b[31m[{}] [ERROR] {}\x1b[0m", $crate::logger::get_local(), $msg);
        $crate::logger::write_log("ERROR", &$msg.to_string());
    }};
}

#[macro_export]
macro_rules! cache {
    ($msg:expr) => {{
        println!("\x1b[95m[{}] [CACHE] {}\x1b[0m", $crate::logger::get_local(), $msg);
        $crate::logger::write_log("CACHE", &$msg.to_string());
    }};
}