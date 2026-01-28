use std::io::Read;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

/// Check if QEMU failed quickly due to acceleration issues.
pub fn is_accel_failure(
    qemu_path: &Path,
    argv: &[std::ffi::OsString],
    grace_period: Duration,
) -> bool {
    let mut cmd = Command::new(qemu_path);
    cmd.args(argv);
    cmd.stderr(Stdio::piped());
    
    let start = Instant::now();
    if let Ok(mut child) = cmd.spawn() {
        // Wait for process to exit or grace period
        let mut stderr = String::new();
        if let Some(mut stderr_handle) = child.stderr.take() {
            let _ = stderr_handle.read_to_string(&mut stderr);
        }
        
        // Wait with timeout
        let mut waited = false;
        while start.elapsed() < grace_period {
            if let Ok(Some(status)) = child.try_wait() {
                waited = true;
                if !status.success() {
                    // Check stderr for accel-related errors
                    let stderr_lower = stderr.to_lowercase();
                    return stderr_lower.contains("whpx")
                        || stderr_lower.contains("failed to initialize whpx")
                        || (stderr_lower.contains("acceleration") && stderr_lower.contains("not available"))
                        || stderr_lower.contains("no accelerator found");
                }
                break;
            }
            std::thread::sleep(Duration::from_millis(100));
        }
        
        if waited {
            // Process exited within grace period with error
            return true;
        }
        
        // Kill if still running
        let _ = child.kill();
    }
    
    false
}
