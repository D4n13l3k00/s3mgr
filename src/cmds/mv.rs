use crate::s3::S3Client;
use crate::utils::colors;
use anyhow::{Context, Result};
use std::path::PathBuf;

pub async fn execute(source: PathBuf, destination: PathBuf, s3_client: &S3Client) -> Result<()> {
    s3_client
        .move_object(
            source.to_str().context("Invalid source path")?,
            destination.to_str().context("Invalid destination path")?,
        )
        .await?;
    println!(
        "`{}` {} `{}`",
        colors::fmt_path(source.to_str().context("Invalid source path")?),
        colors::fmt_success("moved to"),
        colors::fmt_path(destination.to_str().context("Invalid destination path")?)
    );
    Ok(())
}
