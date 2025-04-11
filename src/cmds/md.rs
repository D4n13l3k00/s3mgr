use crate::s3::S3Client;
use crate::utils::colors;
use anyhow::Result;
use std::path::PathBuf;

pub async fn execute(path: PathBuf, s3_client: &S3Client) -> Result<()> {
    let path_str = path.to_string_lossy();

    let key = if path_str.ends_with('/') {
        path_str.to_string()
    } else {
        format!("{}/", path_str)
    };

    s3_client.put_empty_object(&key).await?;
    println!(
        "{} `{}` {}",
        colors::fmt_success("Directory"),
        colors::fmt_path(&key),
        colors::fmt_success("created successfully")
    );

    Ok(())
}
