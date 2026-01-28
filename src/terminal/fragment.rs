use crate::config::schema::{ResolvedConfig, TerminalMode};
use crate::config::paths::get_fragment_file;
use crate::terminal::guid::generate_profile_guid;
use crate::util::fs_atomic;
use serde_json::json;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FragmentError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Atomic write error: {0}")]
    AtomicWrite(#[from] crate::util::fs_atomic::AtomicWriteError),
}

/// Generate fragment JSON for a VM.
pub fn generate_fragment(config: &ResolvedConfig) -> Result<String, FragmentError> {
    let guid = generate_profile_guid(config);
    let profile_name = format!("PortaQEMU: {}", config.vm.name);
    
    let commandline = match config.terminal.mode {
        TerminalMode::Ssh => {
            format!(
                "ssh -p {} {}@localhost",
                config.network.ssh_host_port,
                config.vscode.ssh_user
            )
        }
        TerminalMode::UpAttach => {
            // For up_attach mode, we need the portaqemu.exe path
            // This would need to be passed in or computed
            format!(
                "\"{}\" up --attach",
                "portaqemu.exe" // TODO: resolve actual path
            )
        }
    };
    
    let icon_path = config.terminal.icon.to_string_lossy().replace('\\', "\\\\");
    
    let fragment = json!({
        "profiles": {
            "list": [
                {
                    "guid": guid.to_string(),
                    "name": profile_name,
                    "commandline": commandline,
                    "icon": icon_path,
                    "startingDirectory": "%USERPROFILE%",
                    "source": "PortaQEMU"
                }
            ]
        }
    });
    
    Ok(serde_json::to_string_pretty(&fragment)?)
}

/// Install terminal fragment.
pub fn install_fragment(config: &ResolvedConfig) -> Result<PathBuf, FragmentError> {
    let fragment_file = get_fragment_file(&config.vm.name);
    let fragment_json = generate_fragment(config)?;
    
    // Ensure parent directory exists
    if let Some(parent) = fragment_file.parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    fs_atomic::atomic_write_str(&fragment_file, &fragment_json)?;
    
    Ok(fragment_file)
}

/// Remove terminal fragment.
pub fn remove_fragment(vm_name: &str) -> Result<(), FragmentError> {
    let fragment_file = get_fragment_file(vm_name);
    
    if fragment_file.exists() {
        std::fs::remove_file(&fragment_file)?;
    }
    
    Ok(())
}
