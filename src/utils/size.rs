use anyhow::{anyhow, Result};
use std::str::FromStr;

pub fn parse_human_size(s: &str) -> Result<usize> {
    let s = s.trim().to_uppercase();

    if s.is_empty() {
        return Err(anyhow!("Size cannot be empty"));
    }

    let unit = if s.ends_with("B") {
        if s.len() > 1 {
            s.chars().nth(s.len() - 2).unwrap()
        } else {
            'B'
        }
    } else {
        s.chars().last().unwrap()
    };

    let num_part = if "BKMG".contains(unit) {
        if s.ends_with("B") {
            &s[0..s.len() - 2]
        } else {
            &s[0..s.len() - 1]
        }
    } else {
        &s
    };

    let num = match f64::from_str(num_part) {
        Ok(n) => n,
        Err(_) => return Err(anyhow!("Invalid number: {}", num_part)),
    };

    let bytes = match unit {
        'K' => num * 1024.0,
        'M' => num * 1024.0 * 1024.0,
        'G' => num * 1024.0 * 1024.0 * 1024.0,
        'B' | _ => num,
    };

    if bytes < 0.0 || bytes > (usize::MAX as f64) {
        return Err(anyhow!("Size out of range: {}", s));
    }

    Ok(bytes as usize)
}

pub fn format_size(size: u64) -> String {
    if size < 1024 {
        format!("{} B", size)
    } else if size < 1024 * 1024 {
        format!("{:.2} KB", size as f64 / 1024.0)
    } else if size < 1024 * 1024 * 1024 {
        format!("{:.2} MB", size as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.2} GB", size as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}
