use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VarError {
    #[error("Unresolved variable: {0}")]
    Unresolved(String),
    #[error("Invalid variable syntax: {0}")]
    InvalidSyntax(String),
}

/// Resolve path variables in a string.
/// v0.1 supports only %ROOT%
pub fn resolve_vars(input: &str, root: &Path) -> Result<String, VarError> {
    let mut result = input.to_string();
    let root_str = root.to_string_lossy().replace('\\', "/");
    
    // Replace %ROOT%
    while result.contains("%ROOT%") {
        result = result.replace("%ROOT%", &root_str);
    }
    
    // Check for any remaining unresolved variables
    if result.contains('%') {
        // Try to extract the variable name
        if let Some(start) = result.find('%') {
            let after = &result[start + 1..];
            if let Some(end) = after.find('%') {
                let var_name = &after[..end];
                return Err(VarError::Unresolved(var_name.to_string()));
            }
            return Err(VarError::InvalidSyntax(result));
        }
    }
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_resolve_root() {
        let root = PathBuf::from("/test/root");
        assert_eq!(
            resolve_vars("%ROOT%/vm/disk.qcow2", &root).unwrap(),
            "/test/root/vm/disk.qcow2"
        );
    }

    #[test]
    fn test_resolve_unresolved() {
        let root = PathBuf::from("/test/root");
        assert!(resolve_vars("%UNKNOWN%/file", &root).is_err());
    }
}
