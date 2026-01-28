use std::env;
use std::path::PathBuf;

/// Get the Windows Terminal fragments directory.
pub fn get_fragment_root() -> PathBuf {
    #[cfg(windows)]
    {
        if let Ok(local_app_data) = env::var("LOCALAPPDATA") {
            return PathBuf::from(local_app_data)
                .join("Microsoft")
                .join("Windows Terminal")
                .join("Fragments");
        }
    }
    
    // Fallback for non-Windows (testing)
    if let Ok(home) = env::var("HOME") {
        return PathBuf::from(home)
            .join(".local")
            .join("share")
            .join("WindowsTerminal")
            .join("Fragments");
    }
    
    PathBuf::from(".").join("Fragments")
}

/// Get the fragment file path for a VM.
pub fn get_fragment_file(vm_name: &str) -> PathBuf {
    get_fragment_root()
        .join("PortaQEMU")
        .join(format!("{}.json", vm_name))
}
