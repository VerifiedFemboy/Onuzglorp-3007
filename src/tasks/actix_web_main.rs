use std::sync::Arc;

use actix_web::{App, HttpServer};
use tokio::{spawn, sync::Mutex};

use crate::cache_manager::CacheManager;

pub async fn run_task(
    cache_manager: &Arc<Mutex<CacheManager>>,
) -> std::result::Result<(), std::io::Error> {
    spawn(async move {
        HttpServer::new(|| {
            App::new().route("/", actix_web::web::get().to(|| async { "Hello, world!" }))
        })
        .bind("127.0.1:1337")
        .expect("Failed to bind server")
        .run()
        .await
        .unwrap_or_else(|e| eprintln!("Server error: {}", e));
    });

    Ok(())
}
