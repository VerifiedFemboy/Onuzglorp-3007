use actix_web::{App, HttpServer};


pub async fn run_task() -> std::result::Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new().route("/", actix_web::web::get().to(|| async { "Hello, world!" }))
    })
    .bind("127.0.1:1337")
    .expect("Failed to bind server").run().await?;

    Ok(())
}