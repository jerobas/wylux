use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmState {
    pub running: bool,
    pub qemu_pid: Option<u32>,
    pub started_at: Option<String>,
    pub qemu_args_hash: Option<String>,
    pub last_error: Option<String>,
}

impl Default for VmState {
    fn default() -> Self {
        Self {
            running: false,
            qemu_pid: None,
            started_at: None,
            qemu_args_hash: None,
            last_error: None,
        }
    }
}
