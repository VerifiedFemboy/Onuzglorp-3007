use std::sync::Arc;

use tokio::{spawn, sync::Mutex};

use crate::{LogLevel, cache_manager::CacheManager, log_message};

pub async fn run_task(cache_arc: &Arc<Mutex<CacheManager>>) {
    log_message("Launching cache clearer task", LogLevel::Info);
    let cache_arc = Arc::clone(cache_arc);
    spawn(async move {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(60 * 5)).await;

            let mut cache = cache_arc.lock().await;
            if cache.cache.is_empty() {
                log_message("Cache is empty, nothing to clear.", LogLevel::Info);
                continue;
            }

            log_message("Clearing cache...", LogLevel::Info);
            cache.cleanup_expired();
            log_message(
                &format!("Cache cleared. Current cache size: {}", cache.cache.len()),
                LogLevel::Info,
            );
        }
    });
}
