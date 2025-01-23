use actix_multipart::form::MultipartForm;
use actix_web::{web, App, Error, HttpServer, HttpResponse};
use log;
use std::path::PathBuf;
use std::env;

mod config;
mod error;
mod storage;

use config::Settings;
use storage::{Storage, UploadForm};

async fn upload(
    form: MultipartForm<UploadForm>,
    storage: web::Data<Storage>,
    token: web::Data<String>,
) -> Result<String, Error> {
    log::info!("Received upload request");
    
    // 验证上传 token
    if form.token.0 != *token.as_ref() {
        return Err(actix_web::error::ErrorUnauthorized("Invalid token"));
    }

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

    // 获取上传 token
    let upload_token = env::var("OSS_RS_UPLOAD_TOKEN")
        .expect("Environment variable 'OSS_RS_UPLOAD_TOKEN' not set");
    let upload_token = web::Data::new(upload_token);

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
            .app_data(upload_token.clone())
            .service(web::resource("/upload").route(web::post().to(upload)))
    })
    .bind((settings.server.host, settings.server.port))?
    .run()
    .await
}
