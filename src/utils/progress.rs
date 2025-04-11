use indicatif::{ProgressBar, ProgressStyle};

pub enum ProgressType {
    Download,
    Upload,
    List,
}

pub fn create_progress_bar(size: u64, prefix: &str, progress_type: ProgressType) -> ProgressBar {
    let pb = ProgressBar::new(size);
    let progress_chars = match progress_type {
        ProgressType::Download | ProgressType::Upload => "=>-",
        ProgressType::List => "=>-",
    };

    let style = match progress_type {
        ProgressType::Download | ProgressType::Upload => {
            let template = match progress_type {
                ProgressType::Download => {
                    "[{bar:30.green/red}] {bytes}/{total_bytes} ({eta}) [{bytes_per_sec}] {prefix}"
                }
                ProgressType::Upload => {
                    "[{bar:30.green/red}] {bytes}/{total_bytes} ({eta}) [{bytes_per_sec}] {prefix}"
                }
                _ => unreachable!(),
            };
            ProgressStyle::default_bar()
                .template(template)
                .unwrap()
                .progress_chars(progress_chars)
        }
        ProgressType::List => ProgressStyle::default_bar()
            .template("{spinner:.red} [{bar:30.green/red}] {pos}/{len} ({eta}) {msg}")
            .unwrap()
            .progress_chars(progress_chars),
    };

    pb.set_style(style);
    pb.set_prefix(prefix.to_string());

    if let ProgressType::List = progress_type {
        pb.set_message("Getting file sizes...");
    }

    pb
}

pub fn create_download_progress_bar(size: u64, prefix: &str) -> ProgressBar {
    create_progress_bar(size, prefix, ProgressType::Download)
}

pub fn create_upload_progress_bar(size: u64, prefix: &str) -> ProgressBar {
    create_progress_bar(size, prefix, ProgressType::Upload)
}

pub fn create_list_progress_bar(total: u64) -> ProgressBar {
    create_progress_bar(total, "", ProgressType::List)
}
