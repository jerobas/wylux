use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AtomicWriteError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Failed to create parent directory")]
    ParentDir,
}

/// Atomically write a file by writing to a temp file first, then renaming.
pub fn atomic_write<P: AsRef<Path>>(path: P, contents: &[u8]) -> Result<(), AtomicWriteError> {
    let path = path.as_ref();
    
    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|_| AtomicWriteError::ParentDir)?;
    }

    // Create temp file in same directory
    let temp_path = {
        let mut temp = path.to_path_buf();
        let file_name = temp.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("temp");
        temp.set_file_name(format!(".{}.tmp", file_name));
        temp
    };

    // Write to temp file
    {
        let mut file = fs::File::create(&temp_path)?;
        file.write_all(contents)?;
        file.sync_all()?;
    }

    // Atomic rename
    fs::rename(&temp_path, path)?;

    Ok(())
}

/// Atomically write a string to a file.
pub fn atomic_write_str<P: AsRef<Path>>(path: P, contents: &str) -> Result<(), AtomicWriteError> {
    atomic_write(path, contents.as_bytes())
}
