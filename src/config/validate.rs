use crate::config::schema::ResolvedConfig;
use std::fs;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Disk image not found: {0}")]
    DiskNotFound(String),
    #[error("Disk image not readable: {0}")]
    DiskNotReadable(String),
    #[error("Invalid memory size: {0} MB (must be > 0)")]
    InvalidMemory(u32),
    #[error("Invalid CPU count: {0} (must be > 0)")]
    InvalidCpus(u32),
    #[error("Invalid SSH host port: {0} (must be 1-65535)")]
    InvalidSshPort(u16),
    #[error("Invalid port forward: host={0}, guest={1} (must be 1-65535)")]
    InvalidPortForward(u16, u16),
    #[error("Duplicate host port: {0}")]
    DuplicateHostPort(u16),
}

/// Validate resolved configuration.
pub fn validate_config(config: &ResolvedConfig) -> Result<(), ValidationError> {
    // Validate VM config
    if config.vm.memory_mb == 0 {
        return Err(ValidationError::InvalidMemory(config.vm.memory_mb));
    }
    if config.vm.cpus == 0 {
        return Err(ValidationError::InvalidCpus(config.vm.cpus));
    }
    
    // Validate disk
    if !config.vm.disk.exists() {
        return Err(ValidationError::DiskNotFound(
            config.vm.disk.to_string_lossy().to_string(),
        ));
    }
    if fs::metadata(&config.vm.disk).is_err() {
        return Err(ValidationError::DiskNotReadable(
            config.vm.disk.to_string_lossy().to_string(),
        ));
    }
    
    // Validate network
    if config.network.ssh_host_port == 0 {
        return Err(ValidationError::InvalidSshPort(config.network.ssh_host_port));
    }
    
    // Validate port forwards
    let mut host_ports = vec![config.network.ssh_host_port];
    for forward in &config.network.forwards {
        if forward.host == 0 || forward.guest == 0 {
            return Err(ValidationError::InvalidPortForward(forward.host, forward.guest));
        }
        if host_ports.contains(&forward.host) {
            return Err(ValidationError::DuplicateHostPort(forward.host));
        }
        host_ports.push(forward.host);
    }
    
    Ok(())
}
