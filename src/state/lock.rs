use std::fs;
use std::io::{self, Write};
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LockError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Lock already held by another process")]
    AlreadyLocked,
}

/// Simple file-based lock for preventing concurrent operations.
pub struct Lock {
    path: std::path::PathBuf,
}

impl Lock {
    /// Try to acquire a lock. Returns error if already locked.
    pub fn try_acquire<P: AsRef<Path>>(lock_path: P) -> Result<Self, LockError> {
        let path = lock_path.as_ref();
        
        // Check if lock exists
        if path.exists() {
            // Try to read PID from lock file
            if let Ok(contents) = fs::read_to_string(path) {
                if let Ok(pid) = contents.trim().parse::<u32>() {
                    // Check if process is still running
                    #[cfg(windows)]
                    {
                        use std::os::windows::process::CommandExt;
                        use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION};
                        use windows::Win32::Foundation::CloseHandle;
                        
                        unsafe {
                            let handle = OpenProcess(PROCESS_QUERY_INFORMATION, false, pid);
                            if handle.is_ok() {
                                let _ = CloseHandle(handle.unwrap());
                                return Err(LockError::AlreadyLocked);
                            }
                        }
                    }
                    
                    #[cfg(not(windows))]
                    {
                        // On Unix, check if process exists
                        use std::process::Command;
                        let output = Command::new("kill")
                            .args(&["-0", &pid.to_string()])
                            .output();
                        if output.is_ok() && output.unwrap().status.success() {
                            return Err(LockError::AlreadyLocked);
                        }
                    }
                }
            }
            // Lock file exists but process is dead, remove it
            let _ = fs::remove_file(path);
        }
        
        // Create lock file with current PID
        let pid = std::process::id();
        let mut file = fs::File::create(path)?;
        write!(file, "{}", pid)?;
        file.sync_all()?;
        
        Ok(Self {
            path: path.to_path_buf(),
        })
    }
}

impl Drop for Lock {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}
