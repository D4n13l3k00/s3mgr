use crate::s3::S3Client;
use crate::utils::colors;
use anyhow::Result;
use std::path::PathBuf;

pub async fn execute(path: PathBuf, s3_client: &S3Client) -> Result<()> {
    let path_str = match path.to_str() {
        Some(s) => s,
        None => {
            println!(
                "{}",
                colors::fmt_error(&format!("Invalid path: {}", path.display()))
            );
            return Ok(());
        }
    };

    let is_exists = s3_client.is_exists(path_str).await?;

    if !is_exists {
        println!(
            "{}",
            colors::fmt_error(&format!("File not found: {}", path_str))
        );
        return Ok(());
    }

    let is_dir = s3_client.is_directory(path_str).await?;

    if is_dir {
        println!(
            "{}",
            colors::fmt_error(&format!("{} is a directory", path_str))
        );
        return Ok(());
    }

    let content = s3_client.cat(path_str).await?;
    println!("{}", content);

    Ok(())
}
