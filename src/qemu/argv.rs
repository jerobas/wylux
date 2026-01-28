use crate::config::schema::ResolvedConfig;
use crate::qemu::accel::AccelChoice;
use std::ffi::OsString;
use std::path::Path;

/// Build QEMU command line arguments from config.
pub fn build_argv(
    cfg: &ResolvedConfig,
    qemu_path: &Path,
    accel: AccelChoice,
) -> Vec<OsString> {
    let mut argv = Vec::new();
    
    // QEMU executable (will be prepended by spawn)
    // argv.push(qemu_path.as_os_str().to_os_string());
    
    // Name
    argv.push("-name".into());
    argv.push(cfg.vm.name.clone().into());
    
    // Machine
    argv.push("-machine".into());
    argv.push("q35".into());
    
    // Acceleration
    argv.push("-accel".into());
    argv.push(match accel {
        AccelChoice::Whpx => "whpx".into(),
        AccelChoice::Tcg => "tcg".into(),
    });
    
    // CPU
    argv.push("-cpu".into());
    argv.push(match accel {
        AccelChoice::Whpx => "host".into(),
        AccelChoice::Tcg => "qemu64".into(),
    });
    
    // Memory
    argv.push("-m".into());
    argv.push(cfg.vm.memory_mb.to_string().into());
    
    // CPUs
    argv.push("-smp".into());
    argv.push(cfg.vm.cpus.to_string().into());
    
    // RTC
    argv.push("-rtc".into());
    argv.push("base=localtime".into());
    
    // USB devices
    argv.push("-device".into());
    argv.push("qemu-xhci".into());
    
    argv.push("-device".into());
    argv.push("usb-tablet".into());
    
    // Disk
    let disk_format = detect_disk_format(&cfg.vm.disk);
    argv.push("-drive".into());
    argv.push(format!(
        "file={},if=virtio,format={}",
        cfg.vm.disk.to_string_lossy(),
        disk_format
    ).into());
    
    // Networking
    let mut hostfwd_rules = Vec::new();
    
    // SSH forward (mandatory)
    hostfwd_rules.push(format!(
        "hostfwd=tcp:127.0.0.1:{}-:22",
        cfg.network.ssh_host_port
    ));
    
    // Additional forwards
    for forward in &cfg.network.forwards {
        hostfwd_rules.push(format!(
            "hostfwd=tcp:127.0.0.1:{}-:{}",
            forward.host, forward.guest
        ));
    }
    
    argv.push("-netdev".into());
    argv.push(format!("user,id=n0,{}", hostfwd_rules.join(",")).into());
    
    argv.push("-device".into());
    argv.push("virtio-net-pci,netdev=n0".into());
    
    argv
}

fn detect_disk_format(disk_path: &Path) -> &'static str {
    let ext = disk_path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    
    match ext.as_str() {
        "qcow2" => "qcow2",
        "raw" | "img" => "raw",
        _ => "qcow2", // Default
    }
}
