use std::fs::File;
use std::path::Path;
use std::process::Stdio;
use thiserror::Error;

#[derive(Debug)]
pub struct RunningVm {
    pub pid: u32,
    pub child: Option<std::process::Child>,
}

#[derive(Error, Debug)]
pub enum SpawnError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to spawn QEMU process")]
    SpawnFailed,
}

/// Spawn QEMU process with log redirection.
pub fn spawn_qemu(
    qemu_path: &Path,
    argv: &[std::ffi::OsString],
    log_file: &Path,
) -> Result<RunningVm, SpawnError> {
    // Ensure log directory exists
    if let Some(parent) = log_file.parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    // Open log file
    let log_file_handle = File::create(log_file)?;
    
    // Build command
    let mut cmd = Command::new(qemu_path);
    cmd.args(argv);
    cmd.stdout(Stdio::from(log_file_handle.try_clone()?));
    cmd.stderr(Stdio::from(log_file_handle));
    
    // Spawn
    let mut child = cmd.spawn()?;
    
    // Get PID
    let pid = child.id();
    
    Ok(RunningVm {
        pid,
        child: Some(child),
    })
}
