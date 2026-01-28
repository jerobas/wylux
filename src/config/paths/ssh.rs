use std::path::PathBuf;

/// Get SSH directory path within PortaQEMU root.
pub fn get_ssh_dir(root: &PathBuf) -> PathBuf {
    root.join("config").join("ssh")
}

/// Get SSH identity file path.
pub fn get_identity_file(root: &PathBuf, name: &str) -> PathBuf {
    get_ssh_dir(root).join(name)
}
