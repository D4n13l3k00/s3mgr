use crate::s3::S3Client;
use crate::utils::colors;
use anyhow::{Context, Result};
use std::path::PathBuf;

pub async fn execute(path: PathBuf, recursive: bool, s3_client: &S3Client) -> Result<()> {
    let path_str = path.to_str().context("Invalid path")?;
    let is_dir = s3_client.is_directory(path_str).await?;
    let objects = s3_client.list(Some(path_str)).await?;
    if is_dir && !recursive && !objects.is_empty() {
        println!(
            "{}: cannot remove '{}': Is a directory\nUse -r flag to remove directories",
            colors::fmt_warn("Error"),
            colors::fmt_path(path_str)
        );
        return Ok(());
    }

    if recursive {
        let files = s3_client.list(Some(path_str)).await?;
        for (key, _) in files {
            s3_client.delete(&key).await?;
        }
    } else {
        s3_client.delete(path_str).await?;
    }
    println!(
        "`{}` {}",
        colors::fmt_path(path_str),
        colors::fmt_success("removed successfully")
    );
    Ok(())
}
