// Human-readable output formatting helpers

pub fn format_status(running: bool, pid: Option<u32>) -> String {
    if running {
        if let Some(pid) = pid {
            format!("Running (PID: {})", pid)
        } else {
            "Running".to_string()
        }
    } else {
        "Stopped".to_string()
    }
}

pub fn format_error(msg: &str) -> String {
    format!("Error: {}", msg)
}

pub fn format_success(msg: &str) -> String {
    format!("✓ {}", msg)
}
