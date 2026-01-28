use crate::state::model::VmState;
use crate::util::fs_atomic;
use serde_json;
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StateError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Atomic write error: {0}")]
    AtomicWrite(#[from] crate::util::fs_atomic::AtomicWriteError),
}

/// Load VM state from file.
pub fn load_state<P: AsRef<Path>>(path: P) -> Result<VmState, StateError> {
    let path = path.as_ref();
    if !path.exists() {
        return Ok(VmState::default());
    }
    
    let contents = fs::read_to_string(path)?;
    let state: VmState = serde_json::from_str(&contents)?;
    Ok(state)
}

/// Save VM state atomically.
pub fn save_state<P: AsRef<Path>>(path: P, state: &VmState) -> Result<(), StateError> {
    let json = serde_json::to_string_pretty(state)?;
    fs_atomic::atomic_write_str(path, &json)?;
    Ok(())
}
