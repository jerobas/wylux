use crate::cli::AppContext;
use crate::config::load::load_config;
use crate::qemu::{locate_qemu, detect_available_accels, choose_accel, build_argv, spawn_qemu};
use crate::qemu::probe::is_accel_failure;
use crate::state::{load_state, save_state, model::VmState};
use crate::state::lock::Lock;
use crate::util::net::{check_ports_available, wait_for_port};
use crate::util::hashing::hash_argv;
use crate::util::time::now_iso;
use std::time::Duration;

pub fn handle_up(ctx: &AppContext, _attach: bool, no_wait: bool) -> Result<i32, anyhow::Error> {
    // Acquire lock
    let _lock = Lock::try_acquire(ctx.root.join("config").join("portaqemu.lock"))
        .map_err(|e| anyhow::anyhow!("Failed to acquire lock: {}", e))?;
    
    // Check if already running
    let state_path = ctx.root.join("config").join("state.json");
    let mut state = load_state(&state_path)?;
    if state.running {
        if let Some(pid) = state.qemu_pid {
            use crate::util::process::is_process_running;
            if is_process_running(pid) {
                println!("VM is already running (PID: {})", pid);
                return Ok(0);
            }
        }
    }
    
    // Load config
    let config = load_config(&ctx.config_path, &ctx.root)?;
    
    // Check ports
    let mut ports_to_check = vec![config.network.ssh_host_port];
    for forward in &config.network.forwards {
        ports_to_check.push(forward.host);
    }
    check_ports_available(&ports_to_check)?;
    
    // Locate QEMU
    let qemu_path = locate_qemu(&ctx.root)?;
    
    // Detect acceleration
    let availability = detect_available_accels(&qemu_path);
    let mut accel = choose_accel(config.accel.preferred, &availability)?;
    
    // Build argv
    let mut argv = build_argv(&config, &qemu_path, accel);
    
    // Spawn QEMU
    let log_file = ctx.root.join("logs").join("qemu.log");
    let mut vm = spawn_qemu(&qemu_path, &argv, &log_file)?;
    
    // If auto mode and WHPX fails, retry with TCG
    if config.accel.preferred == crate::config::schema::AccelPreferred::Auto
        && accel == crate::qemu::accel::AccelChoice::Whpx
    {
        if is_accel_failure(&qemu_path, &argv, Duration::from_secs(3)) {
            println!("WHPX failed, retrying with TCG...");
            accel = crate::qemu::accel::AccelChoice::Tcg;
            argv = build_argv(&config, &qemu_path, accel);
            
            // Kill the failed process
            use crate::util::process::kill_process;
            let _ = kill_process(vm.pid);
            
            // Retry spawn
            vm = spawn_qemu(&qemu_path, &argv, &log_file)?;
        }
    }
    
    // Update state
    state.running = true;
    state.qemu_pid = Some(vm.pid);
    state.started_at = Some(now_iso());
    state.qemu_args_hash = Some(hash_argv(&argv));
    state.last_error = None;
    save_state(&state_path, &state)?;
    
    println!("VM started (PID: {})", vm.pid);
    
    // Wait for SSH if requested
    if !no_wait {
        println!("Waiting for SSH to be ready...");
        wait_for_port("127.0.0.1", config.network.ssh_host_port, Duration::from_secs(30))?;
        println!("SSH is ready!");
    }
    
    Ok(0)
}
