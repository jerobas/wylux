use std::process::Command;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProcessError {
    #[error("Process not found")]
    NotFound,
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Process {0} is not running")]
    NotRunning(u32),
}

/// Check if a process is running by PID.
pub fn is_process_running(pid: u32) -> bool {
    #[cfg(windows)]
    {
        // On Windows, try to open the process handle
        // If it fails, the process doesn't exist
        unsafe {
            use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION};
            use windows::Win32::Foundation::CloseHandle;
            
            let handle = OpenProcess(PROCESS_QUERY_INFORMATION, false, pid);
            if let Ok(h) = handle {
                let _ = CloseHandle(h);
                true
            } else {
                false
            }
        }
    }
    
    #[cfg(not(windows))]
    {
        // On Unix, send signal 0 to check if process exists
        let result = Command::new("kill")
            .args(&["-0", &pid.to_string()])
            .output();
        result.is_ok() && result.unwrap().status.success()
    }
}

/// Kill a process by PID (best-effort).
pub fn kill_process(pid: u32) -> Result<(), ProcessError> {
    if !is_process_running(pid) {
        return Err(ProcessError::NotRunning(pid));
    }

    #[cfg(windows)]
    {
        unsafe {
            use windows::Win32::System::Threading::{OpenProcess, TerminateProcess, PROCESS_TERMINATE};
            use windows::Win32::Foundation::CloseHandle;
            
            let handle = OpenProcess(PROCESS_TERMINATE, false, pid)
                .map_err(|_| ProcessError::NotFound)?;
            
            let result = TerminateProcess(handle, 1);
            let _ = CloseHandle(handle);
            result.map_err(|_| ProcessError::Io(std::io::Error::last_os_error()))?;
            Ok(())
        }
    }

    #[cfg(not(windows))]
    {
        Command::new("kill")
            .arg("-9")
            .arg(&pid.to_string())
            .output()
            .map_err(ProcessError::Io)?;
        Ok(())
    }
}
