use crate::config::Config;
use crate::s3::S3Client;
use crate::utils::{colors, progress};
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use tokio::fs;

pub async fn execute(
    path: PathBuf,
    destination: Option<String>,
    recursive: bool,
    chunk_size: Option<usize>,
    s3_client: &S3Client,
) -> Result<()> {
    let config = Config::load()?;
    let chunk_size = chunk_size.unwrap_or(config.upload_chunk_size);

    let metadata = fs::metadata(&path)
        .await
        .context(format!("Failed to get metadata for {}", path.display()))?;

    if metadata.is_dir() {
        if !recursive {
            println!(
                "{}: cannot upload '{}': Is a directory\nUse -r flag to upload directories",
                colors::fmt_warn("Error"),
                colors::fmt_path(&path.to_string_lossy())
            );
            return Ok(());
        }

        let dir_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("directory");

        let s3_destination = match &destination {
            None => Some(dir_name.to_string()),
            Some(dest) => {
                if dest.is_empty() || dest.ends_with('/') {
                    Some(format!("{}{}", dest, dir_name))
                } else {
                    Some(dest.clone())
                }
            }
        };

        upload_directory(&path, s3_destination, "", chunk_size, s3_client).await?;
        println!("{}", colors::fmt_success("Directory uploaded successfully"));
    } else {
        let filename = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("file");

        let s3_path = match destination {
            None => filename.to_string(),
            Some(dest) => {
                if dest.is_empty() || dest.ends_with('/') {
                    format!("{}{}", dest, filename)
                } else {
                    dest
                }
            }
        };

        let file_size = metadata.len();
        let pb = progress::create_upload_progress_bar(
            file_size,
            &format!("{}", colors::fmt_path(&path.to_string_lossy()),),
        );

        s3_client
            .upload_with_progress(&path, &s3_path, chunk_size, |uploaded| {
                pb.set_position(uploaded);
            })
            .await?;

        pb.finish();
    }

    Ok(())
}

async fn upload_directory(
    dir_path: &Path,
    base_destination: Option<String>,
    relative_path: &str,
    chunk_size: usize,
    s3_client: &S3Client,
) -> Result<()> {
    let mut dir = fs::read_dir(dir_path)
        .await
        .context(format!("Failed to read directory {}", dir_path.display()))?;

    let s3_prefix = match &base_destination {
        None => {
            if relative_path.is_empty() {
                String::new()
            } else {
                format!("{}/", relative_path)
            }
        }
        Some(dest) => {
            if dest.is_empty() || dest.ends_with('/') {
                format!("{}{}", dest, relative_path)
            } else if relative_path.is_empty() {
                format!("{}/", dest)
            } else {
                format!("{}/{}", dest, relative_path)
            }
        }
    };

    // Create an empty object to mark the directory
    if !s3_prefix.is_empty() {
        s3_client
            .put_empty_object(&format!("{}/", s3_prefix))
            .await?;
    }

    while let Some(entry) = dir.next_entry().await? {
        let path = entry.path();
        let metadata = fs::metadata(&path).await?;

        let entry_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown");

        if metadata.is_dir() {
            let next_relative_path = if relative_path.is_empty() {
                entry_name.to_string()
            } else {
                format!("{}/{}", relative_path, entry_name)
            };

            let future = upload_directory(
                &path,
                base_destination.clone(),
                &next_relative_path,
                chunk_size,
                s3_client,
            );
            Box::pin(future).await?;
        } else {
            let s3_path = if s3_prefix.is_empty() {
                entry_name.to_string()
            } else {
                format!("{}/{}", s3_prefix, entry_name)
            };

            let file_size = metadata.len();
            let pb = progress::create_upload_progress_bar(
                file_size,
                &format!("{}", colors::fmt_path(&path.to_string_lossy())),
            );

            s3_client
                .upload_with_progress(&path, &s3_path, chunk_size, |uploaded| {
                    pb.set_position(uploaded);
                })
                .await?;

            pb.finish();
        }
    }

    Ok(())
}
