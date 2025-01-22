use crate::error::{AppError, AppResult};
use actix_multipart::form::{
    tempfile::{TempFile, TempFileConfig},
    text::Text,
    MultipartForm,
};
use flate2;
use std::fs;
use std::path::{Path, PathBuf};
use tar;
use zip::ZipArchive;

pub struct Storage {
    root_dir: PathBuf,
}

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    pub file: TempFile,
    pub path: Text<String>,
    pub should_unzip: Text<bool>,
}

impl Storage {
    pub fn new<P: AsRef<Path>>(root_dir: P) -> AppResult<Self> {
        let root_dir = root_dir.as_ref().to_path_buf();
        fs::create_dir_all(&root_dir)?;
        Ok(Self { root_dir })
    }

    pub async fn save_file(&self, form: MultipartForm<UploadForm>) -> AppResult<String> {
        let upload = form.into_inner();

        println!("Starting to process upload request");
        println!("Received path: {}", upload.path.0);
        println!("Should unzip: {}", upload.should_unzip.0);

        let filename = upload
            .file
            .file_name
            .unwrap_or_else(|| "unnamed".to_string());
        let filepath_str = if upload.path.0.is_empty() {
            filename.clone()
        } else {
            format!("{}/{}", upload.path.0.trim().trim_matches('/'), filename)
        };

        println!("Saving to path: {}", filepath_str);
        let file_path: PathBuf = PathBuf::from(filepath_str);
        let save_path = self.root_dir.join(&file_path);

        // 创建目录
        if let Some(parent) = save_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // 保存文件
        upload
            .file
            .file
            .persist(&save_path)
            .map_err(|e| AppError::UploadError(e.to_string()))?;

        // 处理解压
        if upload.should_unzip.0 {
            if let Some(ext) = save_path.extension().and_then(|s| s.to_str()) {
                match ext.to_lowercase().as_str() {
                    "zip" => {
                        self.extract_zip(&save_path, &file_path)?;
                        fs::remove_file(&save_path)?;
                        return Ok(format!(
                            "File uploaded and extracted to {}",
                            file_path.display()
                        ));
                    }
                    "tar" => {
                        self.extract_tar(&save_path, &file_path)?;
                        fs::remove_file(&save_path)?;
                        return Ok(format!(
                            "File uploaded and extracted to {}",
                            file_path.display()
                        ));
                    }
                    "gz" | "tgz" => {
                        self.extract_tar_gz(&save_path, &file_path)?;
                        fs::remove_file(&save_path)?;
                        return Ok(format!(
                            "File uploaded and extracted to {}",
                            file_path.display()
                        ));
                    }
                    _ => {}
                }
            }
        }

        Ok(format!("File uploaded to {}", file_path.display()))
    }

    fn extract_zip<P: AsRef<Path>>(&self, zip_path: P, relative_path: P) -> AppResult<()> {
        let file = fs::File::open(zip_path)?;
        let mut archive = ZipArchive::new(file)?;

        let extract_path = self.root_dir.join(relative_path.as_ref().parent().unwrap());
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let outpath = extract_path.join(file.name());

            if file.name().ends_with('/') {
                fs::create_dir_all(&outpath)?;
            } else {
                if let Some(p) = outpath.parent() {
                    fs::create_dir_all(p)?;
                }
                let mut outfile = fs::File::create(&outpath)?;
                std::io::copy(&mut file, &mut outfile)?;
            }
        }

        Ok(())
    }

    fn extract_tar<P: AsRef<Path>>(&self, tar_path: P, relative_path: P) -> AppResult<()> {
        let file = fs::File::open(tar_path)?;
        let mut archive = tar::Archive::new(file);
        let extract_path = self.root_dir.join(relative_path.as_ref().parent().unwrap());

        archive.unpack(&extract_path)?;
        Ok(())
    }

    fn extract_tar_gz<P: AsRef<Path>>(&self, tar_gz_path: P, relative_path: P) -> AppResult<()> {
        let file = fs::File::open(tar_gz_path)?;
        let decoder = flate2::read::GzDecoder::new(file);
        let mut archive = tar::Archive::new(decoder);
        let extract_path = self.root_dir.join(relative_path.as_ref().parent().unwrap());

        archive.unpack(&extract_path)?;
        Ok(())
    }
}
