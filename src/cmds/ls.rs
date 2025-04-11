use crate::s3::S3Client;
use crate::utils::{colors, progress, size};
use anyhow::Result;
use futures::future;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

struct FileInfo {
    name: String,
    size: u64,
    is_dir: bool,
}

pub async fn execute(path: Option<PathBuf>, s3_client: &S3Client) -> Result<()> {
    let prefix = path.as_deref().and_then(|p| p.to_str());
    let files = s3_client.list(prefix).await?;
    if files.is_empty() {
        println!("{}", colors::fmt_info("No files found"));
        return Ok(());
    }

    let pb = progress::create_list_progress_bar(files.len() as u64);

    let progress = Arc::new(Mutex::new(0));

    let size_futures = files.iter().map(|file| {
        let (key, file_size) = file.clone();
        let pb = pb.clone();
        let progress = Arc::clone(&progress);

        async move {
            let size = if key.ends_with('/') { 0 } else { file_size };

            let mut count = progress.lock().unwrap();
            *count += 1;
            pb.set_position(*count);

            let is_dir = key.ends_with('/');

            FileInfo {
                name: key,
                size,
                is_dir,
            }
        }
    });

    let mut file_infos = future::join_all(size_futures).await;

    pb.finish_and_clear();

    file_infos.sort_by(|a, b| match (a.is_dir, b.is_dir) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.cmp(&b.name),
    });

    for info in &file_infos {
        let size_str = size::format_size(info.size);

        if info.is_dir {
            println!("{:>10}  {}", size_str, colors::fmt_dir_path(&info.name));
        } else {
            println!("{:>10}  {}", size_str, colors::fmt_nested_path(&info.name));
        }
    }

    let total_dirs = file_infos.iter().filter(|info| info.is_dir).count();
    let total_files = file_infos.len() - total_dirs;
    let total_size: u64 = file_infos.iter().map(|info| info.size).sum();

    println!(
        "\n{}: {}",
        colors::fmt_info("Total"),
        size::format_size(total_size)
    );
    println!("{}: {}", colors::fmt_info("Files"), total_files);
    println!("{}: {}", colors::fmt_info("Root dirs"), total_dirs);

    Ok(())
}
