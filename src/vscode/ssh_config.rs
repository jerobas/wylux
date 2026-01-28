use crate::config::schema::ResolvedConfig;
use crate::config::paths::get_default_ssh_config;
use crate::util::fs_atomic;
use std::fs;
use std::path::Path;
use thiserror::Error;

const BEGIN_MARKER: &str = "# PortaQEMU BEGIN";
const END_MARKER: &str = "# PortaQEMU END";

#[derive(Error, Debug)]
pub enum SshConfigError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Config block not found")]
    NotFound,
}

/// Generate SSH config block for VS Code.
pub fn generate_ssh_config_block(config: &ResolvedConfig) -> String {
    let host_name = format!("PortaQEMU-{}", config.vm.name);
    let identity_file = config.vscode.identity_file.to_string_lossy().replace('\\', "/");
    
    format!(
        "Host {}\n\
         \tHostName localhost\n\
         \tPort {}\n\
         \tUser {}\n\
         \tIdentityFile {}\n\
         \tStrictHostKeyChecking no\n\
         \tUserKnownHostsFile /dev/null\n",
        host_name,
        config.network.ssh_host_port,
        config.vscode.ssh_user,
        identity_file
    )
}

/// Print SSH config block.
pub fn print_ssh_config(config: &ResolvedConfig) -> String {
    generate_ssh_config_block(config)
}

/// Install SSH config block into default SSH config.
pub fn install_ssh_config(config: &ResolvedConfig) -> Result<(), SshConfigError> {
    let config_path = get_default_ssh_config();
    
    // Read existing config
    let existing = if config_path.exists() {
        fs::read_to_string(&config_path)?
    } else {
        String::new()
    };
    
    // Remove existing block if present
    let cleaned = remove_block_from_content(&existing);
    
    // Add new block
    let new_content = format!(
        "{}\n{}\n{}\n{}\n",
        cleaned.trim_end(),
        BEGIN_MARKER,
        generate_ssh_config_block(config),
        END_MARKER
    );
    
    // Ensure parent directory exists
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    fs_atomic::atomic_write_str(&config_path, &new_content)?;
    
    Ok(())
}

/// Remove SSH config block from default SSH config.
pub fn remove_ssh_config(config: &ResolvedConfig) -> Result<(), SshConfigError> {
    let config_path = get_default_ssh_config();
    
    if !config_path.exists() {
        return Ok(());
    }
    
    let content = fs::read_to_string(&config_path)?;
    let cleaned = remove_block_from_content(&content);
    
    fs_atomic::atomic_write_str(&config_path, &cleaned)?;
    
    Ok(())
}

fn remove_block_from_content(content: &str) -> String {
    let mut lines = content.lines().collect::<Vec<_>>();
    let mut result = Vec::new();
    let mut in_block = false;
    
    for line in lines {
        if line.trim() == BEGIN_MARKER {
            in_block = true;
            continue;
        }
        if line.trim() == END_MARKER {
            in_block = false;
            continue;
        }
        if !in_block {
            result.push(line);
        }
    }
    
    result.join("\n")
}
