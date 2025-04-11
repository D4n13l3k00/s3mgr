use crate::config::S3Config;
use anyhow::Result;
use s3::creds::Credentials;
use s3::{Bucket, Region};
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;
use tokio::fs;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;

pub struct S3Client {
    bucket: Arc<Bucket>,
}

impl S3Client {
    pub fn new(config: &S3Config) -> Result<Self> {
        let credentials = Credentials::new(
            Some(&config.access_key),
            Some(&config.secret_key),
            None,
            None,
            None,
        )?;

        let region = if let Some(endpoint) = &config.endpoint {
            Region::Custom {
                region: config.region.clone(),
                endpoint: endpoint.clone(),
            }
        } else {
            Region::from_str(&config.region)?
        };

        let bucket = Bucket::new(&config.bucket, region, credentials)?;
        Ok(Self {
            bucket: Arc::new(*bucket),
        })
    }

    pub async fn list(&self, prefix: Option<&str>) -> Result<Vec<(String, u64)>> {
        let prefix = prefix.unwrap_or("");
        let prefix = if prefix.is_empty() {
            "".to_string()
        } else if prefix.ends_with('/') {
            prefix.to_string()
        } else {
            format!("{}/", prefix)
        };

        let objects = self.bucket.list(prefix, None).await?;
        Ok(objects
            .into_iter()
            .flat_map(|obj| obj.contents)
            .map(|content| {
                let key = content.key;
                let size = content.size as u64;
                (key, size)
            })
            .collect())
    }

    pub async fn upload_with_progress<F>(
        &self,
        local_path: &Path,
        s3_path: &str,
        chunk_size: usize,
        progress_callback: F,
    ) -> Result<()>
    where
        F: Fn(u64) + Send + Sync,
    {
        let file_size = fs::metadata(local_path).await?.len();
        let mut file = fs::File::open(local_path).await?;
        let mut uploaded: u64 = 0;
        let mut buffer = vec![0; chunk_size];

        while uploaded < file_size {
            let n = file.read(&mut buffer).await?;
            if n == 0 {
                break;
            }

            let chunk = buffer[..n].to_vec();
            self.bucket.put_object(s3_path, &chunk).await?;

            uploaded += n as u64;
            progress_callback(uploaded);

            if n < chunk_size {
                break;
            }
        }

        Ok(())
    }

    pub async fn delete(&self, path: &str) -> Result<()> {
        self.bucket.delete_object(path).await?;
        Ok(())
    }

    pub async fn move_object(&self, source: &str, destination: &str) -> Result<()> {
        let content = self.bucket.get_object(source).await?;
        let data = content.to_vec();

        self.bucket.put_object(destination, &data).await?;

        self.bucket.delete_object(source).await?;

        Ok(())
    }

    pub async fn copy_object(&self, source: &str, destination: &str) -> Result<()> {
        let content = self.bucket.get_object(source).await?;
        let data = content.to_vec();

        self.bucket.put_object(destination, &data).await?;

        Ok(())
    }

    pub async fn put_empty_object(&self, key: &str) -> Result<()> {
        self.bucket.put_object(key, &[]).await?;
        Ok(())
    }

    pub async fn get_object_size(&self, s3_path: &str) -> Result<u64> {
        let (head, _) = self.bucket.head_object(s3_path).await?;
        Ok(head.content_length.unwrap_or(0).max(0) as u64)
    }

    pub async fn download_with_progress<F>(
        &self,
        s3_path: &str,
        local_path: &Path,
        chunk_size: usize,
        progress_callback: F,
    ) -> Result<()>
    where
        F: Fn(u64) + Send + Sync,
    {
        let content = self.bucket.get_object(s3_path).await?;
        let data = content.to_vec();

        let mut file = fs::File::create(local_path).await?;
        let mut downloaded: u64 = 0;

        let mut offset = 0;
        while offset < data.len() {
            let end = std::cmp::min(offset + chunk_size, data.len());
            let chunk = &data[offset..end];

            file.write_all(chunk).await?;

            downloaded += (end - offset) as u64;
            progress_callback(downloaded);

            offset = end;
        }

        Ok(())
    }

    pub async fn is_directory(&self, path: &str) -> Result<bool> {
        let path = if path.ends_with('/') {
            path.to_string()
        } else {
            format!("{}/", path)
        };

        let objects = self.list(Some(&path)).await?;
        Ok(!objects.is_empty())
    }

    pub async fn list_objects_recursive(&self, prefix: &str) -> Result<Vec<String>> {
        let prefix = if prefix.ends_with('/') {
            prefix.to_string()
        } else {
            format!("{}/", prefix)
        };

        let objects = self.list(Some(&prefix)).await?;
        Ok(objects.into_iter().map(|(key, _)| key).collect())
    }

    pub async fn cat(&self, path: &str) -> Result<String> {
        let content = self.bucket.get_object(path).await?;
        let data = content.to_vec();
        Ok(String::from_utf8(data)?)
    }

    pub async fn is_exists(&self, path: &str) -> Result<bool> {
        match self.bucket.get_object(path).await {
            Ok(_) => Ok(true),
            Err(e) => {
                // if 404 in error, return false
                if e.to_string().contains("404") {
                    Ok(false)
                } else {
                    Err(e.into())
                }
            }
        }
    }
}
