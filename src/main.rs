use actix_multipart::form::MultipartForm;
use actix_web::{web, App, Error, HttpServer};
use log;
use std::path::PathBuf;

mod config;
mod error;
mod storage;

use config::Settings;
use storage::{Storage, UploadForm};

async fn upload(
    form: MultipartForm<UploadForm>,
    storage: web::Data<Storage>,
) -> Result<String, Error> {
    log::info!("Received upload request");
    let result = storage
        .save_file(form)
        .await
        .map_err(|e| actix_web::error::ErrorBadRequest(e.to_string()))?;
    Ok(result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let settings: Settings = Settings::new().expect("Failed to load settings");

    let storage_path = PathBuf::from(&settings.storage_dir)
        .canonicalize()
        .unwrap_or_else(|_| PathBuf::from(&settings.storage_dir));

    log::info!("Storage directory: {:?}", storage_path);

    let storage = web::Data::new(Storage::new(storage_path).expect("Failed to initialize storage"));

    log::info!(
        "Server running at http://{}:{}",
        settings.server.host,
        settings.server.port
    );

    HttpServer::new(move || {
        App::new()
            .app_data(storage.clone())
            .service(web::resource("/upload").route(web::post().to(upload)))
    })
    .bind((settings.server.host, settings.server.port))?
    .run()
    .await
}
