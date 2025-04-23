use env_logger::{Builder, Env};
use log::LevelFilter;
use std::time::Duration;

/// Setup logger with the specified log level
pub fn setup_logger(log_level: &str) {
    let level = match log_level.to_lowercase().as_str() {
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Info,
    };
    
    Builder::from_env(Env::default())
        .filter_level(level)
        .format_timestamp_millis()
        .init();
}

/// Calculate the current query rate based on the number of transactions and elapsed time
pub fn calculate_query_rate(tx_count: usize, elapsed: Duration) -> f64 {
    let elapsed_secs = elapsed.as_secs_f64();
    if elapsed_secs > 0.0 {
        tx_count as f64 / elapsed_secs
    } else {
        0.0
    }
}

/// Format a large number with commas for better readability
#[allow(dead_code)]
pub fn format_number(num: u64) -> String {
    let num_str = num.to_string();
    let mut result = String::new();
    let len = num_str.len();
    
    for (i, c) in num_str.chars().enumerate() {
        result.push(c);
        if (len - i - 1) % 3 == 0 && i < len - 1 {
            result.push(',');
        }
    }
    
    result
}

/// Convert a hex string to a readable address format
#[allow(dead_code)]
pub fn format_address(address: &str) -> String {
    if address.len() < 10 {
        return address.to_string();
    }
    
    // Format as 0x1234...5678
    format!("{}.....{}", &address[0..6], &address[address.len()-4..])
}

/// Helper function to truncate strings that are too long
#[allow(dead_code)]
pub fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}..... ({} chars total)", &s[0..max_len], s.len())
    }
}

/// Create a progress bar string
#[allow(dead_code)]
pub fn progress_bar(percentage: f64, width: usize) -> String {
    let fill_width = (percentage * width as f64 / 100.0).round() as usize;
    let empty_width = width - fill_width;
    
    let fill = "█".repeat(fill_width);
    let empty = "░".repeat(empty_width);
    
    format!("[{}{}] {:.1}%", fill, empty, percentage)
}