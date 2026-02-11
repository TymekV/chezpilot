use std::path::Path;

use indicatif::ProgressStyle;
use owo_colors::OwoColorize;

pub fn make_link(text: &str, url: &str) -> String {
    let visible_text = text.replace(' ', "\u{00A0}");

    format!(
        "\x1b]8;;{url}\x1b\\{}\x1b]8;;\x1b\\",
        visible_text.bright_blue().underline().bold()
    )
}

#[macro_export]
macro_rules! success {
    ($($arg:tt)*) => {
        use tracing::{Level, event};
        event!(target: "success", Level::INFO, $($arg)*);
    };
}

pub fn get_spinner_style() -> ProgressStyle {
    ProgressStyle::with_template("{prefix:.bold.dim}{spinner:.bold.blue} {wide_msg}")
        .unwrap()
        .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
}

pub fn is_hidden(path: &Path) -> bool {
    path.components().any(|comp| {
        if let std::path::Component::Normal(os_str) = comp
            && let Some(str) = os_str.to_str()
        {
            return str.starts_with('.');
        }
        false
    })
}
