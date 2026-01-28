use std::env;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LocateError {
    #[error("QEMU executable not found")]
    NotFound,
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Locate qemu-system-x86_64 executable.
/// Checks: config override -> <root>/bin/qemu/ -> <root>/bin/ -> PATH
pub fn locate_qemu(root: &Path) -> Result<PathBuf, LocateError> {
    // 1. Check <root>/bin/qemu/qemu-system-x86_64.exe
    let qemu_path = root.join("bin").join("qemu").join("qemu-system-x86_64.exe");
    if qemu_path.exists() {
        return Ok(qemu_path);
    }
    
    // 2. Check <root>/bin/qemu-system-x86_64.exe
    let qemu_path = root.join("bin").join("qemu-system-x86_64.exe");
    if qemu_path.exists() {
        return Ok(qemu_path);
    }
    
    // 3. Check PATH
    #[cfg(windows)]
    {
        if let Ok(path) = which::which("qemu-system-x86_64.exe") {
            return Ok(path);
        }
    }
    
    #[cfg(not(windows))]
    {
        if let Ok(path) = which::which("qemu-system-x86_64") {
            return Ok(path);
        }
    }
    
    Err(LocateError::NotFound)
}
