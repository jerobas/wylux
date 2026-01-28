use sha2::{Digest, Sha256};
use std::ffi::OsString;

/// Compute SHA256 hash of QEMU argv for state tracking.
pub fn hash_argv(argv: &[OsString]) -> String {
    let mut hasher = Sha256::new();
    for arg in argv {
        if let Some(s) = arg.to_str() {
            hasher.update(s.as_bytes());
            hasher.update(b"\0");
        }
    }
    format!("{:x}", hasher.finalize())
}
