use crate::cli::AppContext;
use crate::state::{load_state, save_state, model::VmState};
use crate::state::lock::Lock;
use crate::util::process::{is_process_running, kill_process};

pub fn handle_down(ctx: &AppContext) -> Result<i32, anyhow::Error> {
    // Acquire lock
    let _lock = Lock::try_acquire(ctx.root.join("config").join("portaqemu.lock"))
        .map_err(|e| anyhow::anyhow!("Failed to acquire lock: {}", e))?;
    
    let state_path = ctx.root.join("config").join("state.json");
    let mut state = load_state(&state_path)?;
    
    if !state.running {
        println!("VM is not running");
        return Ok(0);
    }
    
    if let Some(pid) = state.qemu_pid {
        if is_process_running(pid) {
            kill_process(pid)?;
            println!("VM stopped (PID: {})", pid);
        } else {
            println!("VM process not found (PID: {})", pid);
        }
    }
    
    // Update state
    state.running = false;
    state.qemu_pid = None;
    save_state(&state_path, &state)?;
    
    Ok(0)
}
