use crate::config::Config;
use crate::utils::{colors, size};
use anyhow::Result;

fn display_config(config: &Config, show_all: bool) {
    println!("{}", colors::fmt_head("Current S3 Configuration:"));
    println!(
        "Access Key ID: {}",
        if config.s3.access_key.is_empty() {
            colors::fmt_val("", "<not set>")
        } else {
            colors::fmt_val(&config.s3.access_key, "<not set>")
        }
    );
    println!(
        "Secret Key: {}",
        if config.s3.secret_key.is_empty() {
            colors::fmt_val("", "<not set>")
        } else if show_all {
            colors::fmt_val(&config.s3.secret_key, "<not set>")
        } else {
            colors::fmt_warn("<hidden>")
        }
    );
    println!("Region: {}", colors::fmt_info(&config.s3.region));
    println!(
        "Bucket: {}",
        if config.s3.bucket.is_empty() {
            colors::fmt_val("", "<not set>")
        } else {
            colors::fmt_val(&config.s3.bucket, "<not set>")
        }
    );
    println!(
        "Endpoint: {}",
        colors::fmt_val(config.s3.endpoint.as_deref().unwrap_or(""), "<not set>")
    );
    println!(
        "Upload Chunk Size: {}",
        colors::fmt_info(&format!(
            "{} ({})",
            size::format_size(config.upload_chunk_size as u64),
            config.upload_chunk_size
        ))
    );
    println!(
        "Download Chunk Size: {}",
        colors::fmt_info(&format!(
            "{} ({})",
            size::format_size(config.download_chunk_size as u64),
            config.download_chunk_size
        ))
    );
}

fn format_chunk_size(size: usize) -> String {
    format!("{} ({})", size::format_size(size as u64), size)
}

fn handle_config_change<T: std::fmt::Display>(
    old_value: T,
    new_value: T,
    field_name: &str,
    show_hidden: bool,
) -> Option<String> {
    if old_value.to_string() != new_value.to_string() {
        Some(format!(
            "{}: {} -> {}",
            field_name,
            if show_hidden {
                colors::fmt_warn(&old_value.to_string())
            } else {
                colors::fmt_warn("<hidden>")
            },
            if show_hidden {
                colors::fmt_success(&new_value.to_string())
            } else {
                colors::fmt_success("<hidden>")
            }
        ))
    } else {
        None
    }
}

pub fn execute(
    access_key: Option<String>,
    secret_key: Option<String>,
    region: Option<String>,
    bucket: Option<String>,
    endpoint: Option<String>,
    upload_chunk_size: Option<usize>,
    download_chunk_size: Option<usize>,
    view: bool,
    show_all: bool,
    reset: bool,
) -> Result<()> {
    if reset {
        Config::reset()?;
        println!(
            "{}",
            colors::fmt_success("Configuration reset to default values")
        );
        return Ok(());
    }

    if show_all && !view {
        println!("{}", colors::fmt_warn("--all works only with -v/--view"));
        return Ok(());
    }

    let mut config = Config::load()?;
    let old_config = config.clone();

    if view {
        display_config(&config, show_all);
        return Ok(());
    }

    let mut changes = Vec::new();

    if let Some(key) = access_key {
        if let Some(change) =
            handle_config_change(&old_config.s3.access_key, &key, "Access Key ID", true)
        {
            changes.push(change);
        }
        config.s3.access_key = key;
    }

    if let Some(key) = secret_key {
        if let Some(change) =
            handle_config_change(&old_config.s3.secret_key, &key, "Secret Key", show_all)
        {
            changes.push(change);
        }
        config.s3.secret_key = key;
    }

    if let Some(reg) = region {
        if let Some(change) = handle_config_change(&old_config.s3.region, &reg, "Region", true) {
            changes.push(change);
        }
        config.s3.region = reg;
    }

    if let Some(bkt) = bucket {
        if let Some(change) = handle_config_change(&old_config.s3.bucket, &bkt, "Bucket", true) {
            changes.push(change);
        }
        config.s3.bucket = bkt;
    }

    if let Some(end) = endpoint {
        let old_end = old_config.s3.endpoint.as_deref().unwrap_or("");
        if let Some(change) = handle_config_change(old_end, &end, "Endpoint", true) {
            changes.push(change);
        }
        config.s3.endpoint = Some(end);
    }

    if let Some(size) = upload_chunk_size {
        if let Some(change) = handle_config_change(
            &format_chunk_size(old_config.upload_chunk_size),
            &format_chunk_size(size),
            "Upload Chunk Size",
            true,
        ) {
            changes.push(change);
        }
        config.upload_chunk_size = size;
    }

    if let Some(size) = download_chunk_size {
        if let Some(change) = handle_config_change(
            &format_chunk_size(old_config.download_chunk_size),
            &format_chunk_size(size),
            "Download Chunk Size",
            true,
        ) {
            changes.push(change);
        }
        config.download_chunk_size = size;
    }

    if !changes.is_empty() {
        println!("{}", colors::fmt_head("Configuration Changes:"));
        for change in changes {
            println!("{}", change);
        }
    }

    config.save()?;
    println!(
        "{}",
        colors::fmt_success("Configuration updated successfully")
    );
    Ok(())
}
