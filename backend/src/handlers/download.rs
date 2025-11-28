use actix_files::NamedFile;
use actix_web::{error, web, Error, HttpRequest, HttpResponse};
use sqlx::PgPool;
use std::io::Write;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

use crate::auth::extract_token_from_header;
use crate::config::Config;
use crate::models::{FileInfo, FilesMetadataResponse, UploaderInfo};
use crate::utils::sanitize_filename_safe;

async fn check_upload_access(
    upload_id: &str,
    pool: &PgPool,
    req: &HttpRequest,
    config: &Config,
) -> Result<(bool, i32), Error> {
    // Get upload info
    let upload_info: Option<(bool, i32)> = sqlx::query_as(
        "SELECT is_available, user_id FROM uploads WHERE upload_id = $1"
    )
    .bind(upload_id)
    .fetch_optional(pool)
    .await
    .map_err(error::ErrorInternalServerError)?;

    let (is_available, uploader_id) = upload_info
        .ok_or_else(|| error::ErrorNotFound("Upload not found"))?;

    // Try to get current user ID
    let current_user_id = extract_token_from_header(
        req.headers()
            .get(actix_web::http::header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok()),
    )
    .or_else(|| req.cookie("auth_token").map(|c| c.value().to_string()))
    .and_then(|token| crate::auth::validate_jwt(&token, &config.jwt_secret).ok())
    .map(|claims| claims.user_id)
    .unwrap_or(-1);

    // Check access
    if !is_available && current_user_id != uploader_id {
        return Err(error::ErrorGone(
            "This file has expired or is no longer available",
        ));
    }

    Ok((is_available, uploader_id))
}

pub async fn download(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    req: HttpRequest,
    upload_id: web::Path<String>,
) -> Result<HttpResponse, Error> {
    check_upload_access(&upload_id, &pool, &req, &config).await?;

    // Find matching files
    let entries = std::fs::read_dir("./uploads")
        .map_err(error::ErrorInternalServerError)?;
    
    let prefix = format!("{}_", upload_id);
    let mut matching_files: Vec<String> = entries
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().to_string())
        .filter(|name| name.starts_with(&prefix))
        .collect();

    if matching_files.is_empty() {
        return Err(error::ErrorNotFound("Files not found"));
    }

    // Single file - serve directly
    if matching_files.len() == 1 {
        let file_path = format!("./uploads/{}", matching_files[0]);
        let file = NamedFile::open(file_path)
            .map_err(error::ErrorInternalServerError)?;
        return Ok(file.into_response(&req));
    }

    // Multiple files - create ZIP
    matching_files.sort();
    let mut buffer = Vec::new();
    {
        let mut zip = ZipWriter::new(std::io::Cursor::new(&mut buffer));
        let options = SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);

        for file_name in &matching_files {
            let original_name = file_name.strip_prefix(&prefix).unwrap_or(file_name);
            let file_path = format!("./uploads/{}", file_name);
            let file_content = std::fs::read(&file_path)
                .map_err(error::ErrorInternalServerError)?;

            zip.start_file(original_name, options)
                .map_err(error::ErrorInternalServerError)?;
            zip.write_all(&file_content)
                .map_err(error::ErrorInternalServerError)?;
        }

        zip.finish().map_err(error::ErrorInternalServerError)?;
    }

    Ok(HttpResponse::Ok()
        .content_type("application/zip")
        .append_header(("Content-Disposition", "attachment; filename=\"files.zip\""))
        .body(buffer))
}

pub async fn download_file(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    req: HttpRequest,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, Error> {
    let (upload_id, filename) = path.into_inner();

    check_upload_access(&upload_id, &pool, &req, &config).await?;

    let sanitized = sanitize_filename_safe(&filename);
    let actual_filename = format!("{}_{}", upload_id, sanitized);
    let file_path = format!("./uploads/{}", actual_filename);

    if !std::path::Path::new(&file_path).exists() {
        return Err(error::ErrorNotFound("File not found"));
    }

    let file = NamedFile::open(file_path)
        .map_err(error::ErrorInternalServerError)?;
    Ok(file.into_response(&req))
}

pub async fn get_file_metadata(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    req: HttpRequest,
    upload_id: web::Path<String>,
) -> Result<HttpResponse, Error> {
    check_upload_access(&upload_id, &pool, &req, &config).await?;

    // Get uploader info
    let uploader_info: Option<(String, Option<String>, Option<String>, Option<chrono::DateTime<chrono::Utc>>)> = sqlx::query_as(
        r#"
        SELECT u.username, u.avatar, up.email, up.expires_at
        FROM uploads up
        JOIN users u ON up.user_id = u.id
        WHERE up.upload_id = $1
        "#
    )
    .bind(upload_id.as_str())
    .fetch_optional(pool.as_ref())
    .await
    .map_err(error::ErrorInternalServerError)?;

    let (username, avatar, email, expiration_date) = uploader_info
        .unwrap_or_else(|| ("Unknown".to_string(), None, None, None));

    // Get files
    let entries = std::fs::read_dir("./uploads")
        .map_err(error::ErrorInternalServerError)?;
    
    let prefix = format!("{}_", upload_id);
    let mut file_infos = Vec::new();

    for entry in entries {
        if let Ok(entry) = entry {
            let file_name = entry.file_name().to_string_lossy().to_string();
            if file_name.starts_with(&prefix) {
                let original_name = file_name.strip_prefix(&prefix).unwrap_or(&file_name);
                if let Ok(metadata) = entry.metadata() {
                    file_infos.push(FileInfo {
                        name: original_name.to_string(),
                        size: metadata.len(),
                        url: format!("/api/file/{}/{}", upload_id, original_name),
                    });
                }
            }
        }
    }

    if file_infos.is_empty() {
        return Err(error::ErrorNotFound("Files not found"));
    }

    Ok(HttpResponse::Ok()
        .insert_header(("Cache-Control", "no-store, no-cache, must-revalidate, proxy-revalidate"))
        .insert_header(("Pragma", "no-cache"))
        .insert_header(("Expires", "0"))
        .json(FilesMetadataResponse {
            files: file_infos,
            uploader: UploaderInfo {
                username,
                avatar: avatar.unwrap_or_default(),
                email,
                expiration_date,
            },
        }))
}
