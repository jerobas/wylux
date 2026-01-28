use crate::config::schema::ResolvedConfig;
use crate::qemu::locate::locate_qemu;
use crate::qemu::accel::{detect_available_accels, choose_accel};
use crate::util::net::is_port_available;
use crate::autostart::is_autostart_enabled;
use crate::terminal::fragment::get_fragment_file;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckStatus {
    Pass,
    Warn,
    Fail,
}

#[derive(Debug, Clone)]
pub struct CheckResult {
    pub id: &'static str,
    pub status: CheckStatus,
    pub message: String,
    pub hint: Option<String>,
}

pub fn check_qemu_binary(root: &PathBuf) -> CheckResult {
    match locate_qemu(root) {
        Ok(path) => CheckResult {
            id: "qemu_binary",
            status: CheckStatus::Pass,
            message: format!("QEMU found: {}", path.to_string_lossy()),
            hint: None,
        },
        Err(_) => CheckResult {
            id: "qemu_binary",
            status: CheckStatus::Fail,
            message: "QEMU binary not found".to_string(),
            hint: Some("Install QEMU or place qemu-system-x86_64.exe in bin/".to_string()),
        },
    }
}

pub fn check_disk_image(config: &ResolvedConfig) -> CheckResult {
    if config.vm.disk.exists() {
        CheckResult {
            id: "disk_image",
            status: CheckStatus::Pass,
            message: format!("Disk image found: {}", config.vm.disk.to_string_lossy()),
            hint: None,
        }
    } else {
        CheckResult {
            id: "disk_image",
            status: CheckStatus::Fail,
            message: format!("Disk image not found: {}", config.vm.disk.to_string_lossy()),
            hint: Some("Create or download a VM disk image".to_string()),
        }
    }
}

pub fn check_acceleration(root: &PathBuf, config: &ResolvedConfig) -> CheckResult {
    if let Ok(qemu_path) = locate_qemu(root) {
        let availability = detect_available_accels(&qemu_path);
        match choose_accel(config.accel.preferred, &availability) {
            Ok(accel) => {
                let accel_name = match accel {
                    crate::qemu::accel::AccelChoice::Whpx => "WHPX",
                    crate::qemu::accel::AccelChoice::Tcg => "TCG",
                };
                CheckResult {
                    id: "acceleration",
                    status: CheckStatus::Pass,
                    message: format!("Acceleration available: {}", accel_name),
                    hint: None,
                }
            }
            Err(_) => CheckResult {
                id: "acceleration",
                status: CheckStatus::Warn,
                message: "Preferred acceleration not available, will use fallback".to_string(),
                hint: Some("Consider installing WHPX or using TCG".to_string()),
            },
        }
    } else {
        CheckResult {
            id: "acceleration",
            status: CheckStatus::Fail,
            message: "Cannot check acceleration: QEMU not found".to_string(),
            hint: None,
        }
    }
}

pub fn check_ports(config: &ResolvedConfig) -> CheckResult {
    let mut ports_to_check = vec![config.network.ssh_host_port];
    for forward in &config.network.forwards {
        ports_to_check.push(forward.host);
    }
    
    let mut in_use = Vec::new();
    for &port in &ports_to_check {
        if let Ok(available) = is_port_available(port) {
            if !available {
                in_use.push(port);
            }
        }
    }
    
    if in_use.is_empty() {
        CheckResult {
            id: "ports",
            status: CheckStatus::Pass,
            message: "All required ports are available".to_string(),
            hint: None,
        }
    } else {
        CheckResult {
            id: "ports",
            status: CheckStatus::Warn,
            message: format!("Ports in use: {:?}", in_use),
            hint: Some("Stop conflicting services or change port configuration".to_string()),
        }
    }
}

pub fn check_ssh_key(config: &ResolvedConfig) -> CheckResult {
    if config.vscode.identity_file.exists() {
        CheckResult {
            id: "ssh_key",
            status: CheckStatus::Pass,
            message: format!("SSH key found: {}", config.vscode.identity_file.to_string_lossy()),
            hint: None,
        }
    } else {
        CheckResult {
            id: "ssh_key",
            status: CheckStatus::Warn,
            message: format!("SSH key not found: {}", config.vscode.identity_file.to_string_lossy()),
            hint: Some("Generate SSH key pair with: ssh-keygen -t ed25519".to_string()),
        }
    }
}

pub fn check_terminal_fragment(config: &ResolvedConfig) -> CheckResult {
    let fragment_file = get_fragment_file(&config.vm.name);
    if fragment_file.exists() {
        CheckResult {
            id: "terminal_fragment",
            status: CheckStatus::Pass,
            message: format!("Terminal fragment installed: {}", fragment_file.to_string_lossy()),
            hint: None,
        }
    } else {
        CheckResult {
            id: "terminal_fragment",
            status: CheckStatus::Warn,
            message: "Terminal fragment not installed".to_string(),
            hint: Some("Run: portaqemu terminal install".to_string()),
        }
    }
}

pub fn check_autostart(config: &ResolvedConfig) -> CheckResult {
    if is_autostart_enabled(&config.vm.name) {
        CheckResult {
            id: "autostart",
            status: CheckStatus::Pass,
            message: "Autostart is enabled".to_string(),
            hint: None,
        }
    } else {
        CheckResult {
            id: "autostart",
            status: CheckStatus::Warn,
            message: "Autostart is not enabled".to_string(),
            hint: Some("Run: portaqemu enable".to_string()),
        }
    }
}

pub fn run_all_checks(config: &ResolvedConfig, root: &PathBuf) -> Vec<CheckResult> {
    vec![
        check_qemu_binary(root),
        check_disk_image(config),
        check_acceleration(root, config),
        check_ports(config),
        check_ssh_key(config),
        check_terminal_fragment(config),
        check_autostart(config),
    ]
}
