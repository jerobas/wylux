use crate::config::schema::*;
use crate::config::vars::resolve_vars;
use crate::config::validate::validate_config;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigLoadError {
    #[error("Failed to read config file: {0}")]
    Read(#[from] std::io::Error),
    #[error("Failed to parse TOML: {0}")]
    Parse(#[from] toml::de::Error),
    #[error("Variable resolution error: {0}")]
    Var(#[from] crate::config::vars::VarError),
    #[error("Validation error: {0}")]
    Validation(#[from] crate::config::validate::ValidationError),
}

/// Load and resolve configuration from a TOML file.
pub fn load_config<P: AsRef<Path>>(config_path: P, root: &Path) -> Result<ResolvedConfig, ConfigLoadError> {
    let config_path = config_path.as_ref();
    let contents = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&contents)?;
    
    // Resolve variables
    let disk_path = resolve_vars(&config.vm.disk, root)?;
    let icon_path = resolve_vars(&config.terminal.icon, root)?;
    let identity_path = resolve_vars(&config.vscode.identity_file, root)?;
    
    // Convert to absolute paths
    let disk = if PathBuf::from(&disk_path).is_absolute() {
        PathBuf::from(disk_path)
    } else {
        root.join(disk_path)
    };
    
    let icon = if PathBuf::from(&icon_path).is_absolute() {
        PathBuf::from(icon_path)
    } else {
        root.join(icon_path)
    };
    
    let identity_file = if PathBuf::from(&identity_path).is_absolute() {
        PathBuf::from(identity_path)
    } else {
        root.join(identity_path)
    };
    
    let resolved = ResolvedConfig {
        vm: ResolvedVmConfig {
            name: config.vm.name,
            disk: disk.canonicalize().unwrap_or(disk),
            memory_mb: config.vm.memory_mb,
            cpus: config.vm.cpus,
        },
        network: config.network,
        accel: config.accel,
        terminal: ResolvedTerminalConfig {
            profile_name: config.terminal.profile_name,
            icon: icon.canonicalize().unwrap_or(icon),
            mode: config.terminal.mode,
        },
        vscode: ResolvedVscodeConfig {
            ssh_user: config.vscode.ssh_user,
            identity_file: identity_file.canonicalize().unwrap_or(identity_file),
        },
    };
    
    // Validate
    validate_config(&resolved)?;
    
    Ok(resolved)
}
