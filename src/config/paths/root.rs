use std::env;
use std::path::PathBuf;

/// Get the PortaQEMU root directory.
/// Checks --root flag, then PORTAQEMU_ROOT env var, then defaults to %LOCALAPPDATA%\PortaQEMU
pub fn get_root(root_override: Option<&PathBuf>) -> PathBuf {
    if let Some(root) = root_override {
        return root.clone();
    }
    
    if let Ok(env_root) = env::var("PORTAQEMU_ROOT") {
        return PathBuf::from(env_root);
    }
    
    #[cfg(windows)]
    {
        if let Ok(local_app_data) = env::var("LOCALAPPDATA") {
            return PathBuf::from(local_app_data).join("PortaQEMU");
        }
    }
    
    #[cfg(not(windows))]
    {
        // Fallback for non-Windows (for development/testing)
        if let Ok(home) = env::var("HOME") {
            return PathBuf::from(home).join(".local").join("share").join("PortaQEMU");
        }
    }
    
    // Last resort
    PathBuf::from(".").join("PortaQEMU")
}
