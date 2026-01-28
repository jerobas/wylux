use crate::cli::AppContext;
use std::fs;

pub fn handle_init(ctx: &AppContext) -> Result<i32, anyhow::Error> {
    // Create directory structure
    let dirs = vec![
        ctx.root.join("bin"),
        ctx.root.join("vm"),
        ctx.root.join("config"),
        ctx.root.join("config").join("ssh"),
        ctx.root.join("logs"),
        ctx.root.join("terminal"),
    ];
    
    for dir in dirs {
        fs::create_dir_all(&dir)?;
    }
    
    // Create default config if it doesn't exist
    let config_path = &ctx.config_path;
    if !config_path.exists() {
        let default_config = r#"[vm]
name = "devvm"
disk = "%ROOT%/vm/disk.qcow2"
memory_mb = 4096
cpus = 4

[network]
ssh_host_port = 2222
forwards = []

[accel]
preferred = "auto"

[terminal]
profile_name = "PortaQEMU Dev VM"
icon = "%ROOT%/bin/icon.ico"
mode = "ssh"

[vscode]
ssh_user = "dev"
identity_file = "%ROOT%/config/ssh/id_ed25519"
"#;
        fs::write(config_path, default_config)?;
    }
    
    println!("Initialized PortaQEMU at: {}", ctx.root.to_string_lossy());
    Ok(0)
}
