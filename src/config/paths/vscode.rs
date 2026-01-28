use std::env;
use std::path::PathBuf;

/// Get the default SSH config path.
pub fn get_default_ssh_config() -> PathBuf {
    #[cfg(windows)]
    {
        if let Ok(user_profile) = env::var("USERPROFILE") {
            return PathBuf::from(user_profile).join(".ssh").join("config");
        }
    }
    
    // Unix fallback
    if let Ok(home) = env::var("HOME") {
        return PathBuf::from(home).join(".ssh").join("config");
    }
    
    PathBuf::from(".").join("ssh_config")
}
