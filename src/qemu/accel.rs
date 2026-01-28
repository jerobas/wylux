use crate::config::schema::{AccelPreferred, ResolvedConfig};
use std::process::Command;
use std::str;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccelChoice {
    Whpx,
    Tcg,
}

#[derive(Debug, Clone)]
pub struct AccelAvailability {
    pub whpx_available: bool,
    pub tcg_available: bool,
}

#[derive(Error, Debug)]
pub enum AccelError {
    #[error("WHPX acceleration not available")]
    WhpxUnavailable,
    #[error("TCG acceleration not available")]
    TcgUnavailable,
    #[error("No acceleration available")]
    NoneAvailable,
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Detect available accelerators by running `qemu-system-x86_64 -accel help`.
pub fn detect_available_accels(qemu_path: &std::path::Path) -> AccelAvailability {
    let output = Command::new(qemu_path)
        .arg("-accel")
        .arg("help")
        .output();
    
    let mut whpx_available = false;
    let mut tcg_available = false;
    
    if let Ok(output) = output {
        let stdout = str::from_utf8(&output.stdout).unwrap_or("");
        let stderr = str::from_utf8(&output.stderr).unwrap_or("");
        let combined = format!("{} {}", stdout, stderr);
        
        whpx_available = combined.contains("whpx");
        tcg_available = combined.contains("tcg");
    }
    
    // TCG is always available as fallback
    if !tcg_available {
        tcg_available = true;
    }
    
    AccelAvailability {
        whpx_available,
        tcg_available,
    }
}

/// Choose acceleration based on preference and availability.
pub fn choose_accel(
    preferred: AccelPreferred,
    availability: &AccelAvailability,
) -> Result<AccelChoice, AccelError> {
    match preferred {
        AccelPreferred::Whpx => {
            if availability.whpx_available {
                Ok(AccelChoice::Whpx)
            } else {
                Err(AccelError::WhpxUnavailable)
            }
        }
        AccelPreferred::Tcg => {
            if availability.tcg_available {
                Ok(AccelChoice::Tcg)
            } else {
                Err(AccelError::TcgUnavailable)
            }
        }
        AccelPreferred::Auto => {
            if availability.whpx_available {
                Ok(AccelChoice::Whpx)
            } else if availability.tcg_available {
                Ok(AccelChoice::Tcg)
            } else {
                Err(AccelError::NoneAvailable)
            }
        }
    }
}
