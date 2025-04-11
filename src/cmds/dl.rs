use crate::config::Config;
use crate::s3::S3Client;
use crate::utils::{colors, progress};
use anyhow::Result;
use std::path::{Path, PathBuf};
use tokio::fs;

pub async fn execute(
    source: String,
    destination: PathBuf,
    recursive: bool,
    chunk_size: Option<usize>,
    s3_client: &S3Client,
) -> Result<()> {
    let config = Config::load()?;
    let chunk_size = chunk_size.unwrap_or(config.download_chunk_size);

    let is_dir = s3_client.is_directory(&source).await?;
    if is_dir && !recursive {
        println!(
            "{}",
            colors::fmt_warn("Source is a directory. Use -r/--recursive to download directories.")
        );
        return Ok(());
    }

    if is_dir {
        let objects = s3_client.list_objects_recursive(&source).await?;
        let source_name = source
            .trim_end_matches('/')
            .split('/')
            .last()
            .unwrap_or(&source);
        for object in objects {
            let relative_path = if object == source {
                source_name.to_string()
            } else {
                let stripped = object
                    .strip_prefix(&source)
                    .unwrap_or(&object)
                    .trim_start_matches('/');
                format!("{}/{}", source_name, stripped)
            };

            let local_path = if destination == Path::new(".") {
                PathBuf::from(relative_path)
            } else {
                destination.join(relative_path)
            };

            if let Some(parent) = local_path.parent() {
                fs::create_dir_all(parent).await?;
            }

            if object.ends_with('/') {
                continue;
            }

            let file_size = s3_client.get_object_size(&object).await?;
            let pb = progress::create_download_progress_bar(
                file_size,
                &format!("{}", colors::fmt_path(&object)),
            );

            s3_client
                .download_with_progress(&object, &local_path, chunk_size, |downloaded| {
                    pb.set_position(downloaded);
                })
                .await?;

            pb.finish();
        }
        return Ok(());
    }

    let file_name = source.split('/').last().unwrap_or(&source);
    let actual_destination = if destination == Path::new(".") {
        PathBuf::from(file_name)
    } else {
        if let Ok(metadata) = fs::metadata(&destination).await {
            if metadata.is_dir() {
                destination.join(file_name)
            } else {
                destination
            }
        } else {
            destination
        }
    };

    let file_size = s3_client.get_object_size(&source).await?;
    let pb = progress::create_download_progress_bar(
        file_size,
        &format!("{}", colors::fmt_path(&source)),
    );

    s3_client
        .download_with_progress(&source, &actual_destination, chunk_size, |downloaded| {
            pb.set_position(downloaded);
        })
        .await?;

    pb.finish();
    Ok(())
}
