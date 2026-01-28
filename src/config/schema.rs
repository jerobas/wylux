use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub vm: VmConfig,
    pub network: NetworkConfig,
    pub accel: AccelConfig,
    pub terminal: TerminalConfig,
    pub vscode: VscodeConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VmConfig {
    pub name: String,
    pub disk: String, // Will be resolved to PathBuf
    pub memory_mb: u32,
    pub cpus: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NetworkConfig {
    pub ssh_host_port: u16,
    #[serde(default)]
    pub forwards: Vec<PortForward>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PortForward {
    pub host: u16,
    pub guest: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AccelConfig {
    #[serde(default = "default_accel_preferred")]
    pub preferred: AccelPreferred,
}

fn default_accel_preferred() -> AccelPreferred {
    AccelPreferred::Auto
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AccelPreferred {
    Auto,
    Whpx,
    Tcg,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TerminalConfig {
    pub profile_name: String,
    pub icon: String, // Will be resolved to PathBuf
    #[serde(default = "default_terminal_mode")]
    pub mode: TerminalMode,
}

fn default_terminal_mode() -> TerminalMode {
    TerminalMode::Ssh
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TerminalMode {
    Ssh,
    UpAttach,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VscodeConfig {
    pub ssh_user: String,
    pub identity_file: String, // Will be resolved to PathBuf
}

/// Resolved configuration with absolute paths.
#[derive(Debug, Clone)]
pub struct ResolvedConfig {
    pub vm: ResolvedVmConfig,
    pub network: NetworkConfig,
    pub accel: AccelConfig,
    pub terminal: ResolvedTerminalConfig,
    pub vscode: ResolvedVscodeConfig,
}

#[derive(Debug, Clone)]
pub struct ResolvedVmConfig {
    pub name: String,
    pub disk: PathBuf,
    pub memory_mb: u32,
    pub cpus: u32,
}

#[derive(Debug, Clone)]
pub struct ResolvedTerminalConfig {
    pub profile_name: String,
    pub icon: PathBuf,
    pub mode: TerminalMode,
}

#[derive(Debug, Clone)]
pub struct ResolvedVscodeConfig {
    pub ssh_user: String,
    pub identity_file: PathBuf,
}
