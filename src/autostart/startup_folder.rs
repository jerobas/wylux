use std::env;
use std::fs;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AutostartError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to determine startup folder")]
    StartupFolderNotFound,
}

/// Get Windows Startup folder path.
pub fn get_startup_folder() -> Result<PathBuf, AutostartError> {
    #[cfg(windows)]
    {
        if let Ok(app_data) = env::var("APPDATA") {
            return Ok(PathBuf::from(app_data)
                .join("Microsoft")
                .join("Windows")
                .join("Start Menu")
                .join("Programs")
                .join("Startup"));
        }
    }
    
    #[cfg(not(windows))]
    {
        // Fallback for non-Windows
        if let Ok(home) = env::var("HOME") {
            return Ok(PathBuf::from(home)
                .join(".config")
                .join("autostart"));
        }
    }
    
    Err(AutostartError::StartupFolderNotFound)
}

/// Get autostart script filename for a VM.
pub fn get_autostart_filename(vm_name: &str) -> String {
    format!("PortaQEMU ({}).cmd", vm_name)
}

/// Enable autostart for a VM.
pub fn enable_autostart(vm_name: &str, root: &PathBuf) -> Result<PathBuf, AutostartError> {
    let startup_folder = get_startup_folder()?;
    let script_name = get_autostart_filename(vm_name);
    let script_path = startup_folder.join(&script_name);
    
    // Get portaqemu.exe path
    let exe_path = root.join("bin").join("portaqemu.exe");
    let exe_str = exe_path.to_string_lossy().replace('/', "\\");
    
    // Create .cmd file
    let script_content = format!(
        "@echo off\n\"{}\" up --no-wait\n",
        exe_str
    );
    
    fs::create_dir_all(&startup_folder)?;
    fs::write(&script_path, script_content)?;
    
    Ok(script_path)
}

/// Disable autostart for a VM.
pub fn disable_autostart(vm_name: &str) -> Result<(), AutostartError> {
    let startup_folder = get_startup_folder()?;
    let script_name = get_autostart_filename(vm_name);
    let script_path = startup_folder.join(&script_name);
    
    if script_path.exists() {
        fs::remove_file(&script_path)?;
    }
    
    Ok(())
}

/// Check if autostart is enabled for a VM.
pub fn is_autostart_enabled(vm_name: &str) -> bool {
    if let Ok(startup_folder) = get_startup_folder() {
        let script_name = get_autostart_filename(vm_name);
        let script_path = startup_folder.join(&script_name);
        return script_path.exists();
    }
    false
}
