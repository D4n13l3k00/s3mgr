use crossterm::style::{Color, Stylize};

pub fn fmt_head(text: &str) -> String {
    format!("{}", text.bold())
}

pub fn fmt_success(text: &str) -> String {
    format!("{}", text.green())
}

pub fn fmt_warn(text: &str) -> String {
    format!("{}", text.yellow())
}

pub fn fmt_info(text: &str) -> String {
    format!("{}", text.blue())
}

pub fn fmt_error(text: &str) -> String {
    format!("{}", text.red())
}

pub fn fmt_val<T: AsRef<str>>(value: T, empty_placeholder: &str) -> String {
    let value = value.as_ref();
    if value.is_empty() {
        format!("{}", empty_placeholder.with(Color::Red))
    } else {
        format!("{}", value.with(Color::Green))
    }
}

pub fn fmt_path(path: &str) -> String {
    if path.ends_with('/') {
        fmt_dir(path)
    } else {
        fmt_file(path)
    }
}

pub fn fmt_file(path: &str) -> String {
    format!("{}", path.with(Color::Cyan))
}

pub fn fmt_dir(path: &str) -> String {
    format!("{}", path.with(Color::DarkBlue))
}

pub fn fmt_nested_path(path: &str) -> String {
    let parts: Vec<&str> = path.split('/').collect();
    if parts.is_empty() {
        return String::new();
    }

    let mut result = String::new();
    let last_index = parts.len() - 1;

    for (i, part) in parts.iter().enumerate() {
        if i > 0 {
            result.push_str(&format!("{}", "/".with(Color::DarkBlue)));
        }

        if !part.is_empty() {
            if i == last_index {
                result.push_str(&format!("{}", part.with(Color::Green)));
            } else {
                result.push_str(&format!("{}", part.with(Color::Cyan)));
            }
        }
    }

    result
}

pub fn fmt_dir_path(path: &str) -> String {
    let parts: Vec<&str> = path.split('/').collect();
    if parts.is_empty() {
        return String::new();
    }

    let mut result = String::new();

    for (i, part) in parts.iter().enumerate() {
        if i > 0 {
            result.push_str(&format!("{}", "/".with(Color::DarkBlue)));
        }

        if !part.is_empty() {
            result.push_str(&format!("{}", part.with(Color::Cyan)));
        }
    }

    result
}
