use std::sync::Arc;

use tokio::{spawn, sync::Mutex};

use crate::{cache, cache_manager::CacheManager, info};

pub async fn run_task(cache_arc: &Arc<Mutex<CacheManager>>) {
    info!("Launching cache clearer task");
    let cache_arc = Arc::clone(cache_arc);
    spawn(async move {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(60 * 5)).await;

            let mut cache = cache_arc.lock().await;
            if cache.cache.is_empty() {
                continue;
            }
            info!("Clearing cache...");
            cache.cleanup_expired();
            cache!(format!(
                "Cache cleared. Current cache size: {}",
                cache.cache.len()
            ));
        }
    });
}
