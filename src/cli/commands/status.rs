use crate::cli::AppContext;
use crate::state::load_state;
use crate::util::process::is_process_running;
use crate::output::{OutputMode, human::format_status};

pub fn handle_status(ctx: &AppContext) -> Result<i32, anyhow::Error> {
    let state_path = ctx.root.join("config").join("state.json");
    let state = load_state(&state_path)?;
    
    let actually_running = state.qemu_pid
        .map(|pid| is_process_running(pid))
        .unwrap_or(false);
    
    match ctx.output_mode {
        OutputMode::Json => {
            use serde_json::json;
            let json = json!({
                "running": actually_running,
                "pid": state.qemu_pid,
                "started_at": state.started_at,
            });
            println!("{}", serde_json::to_string_pretty(&json)?);
        }
        OutputMode::Human => {
            println!("Status: {}", format_status(actually_running, state.qemu_pid));
            if let Some(started_at) = &state.started_at {
                println!("Started at: {}", started_at);
            }
        }
    }
    
    Ok(0)
}
