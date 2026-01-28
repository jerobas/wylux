use crate::cli::AppContext;
use crate::config::load::load_config;
use std::process::Command;

pub fn handle_ssh(ctx: &AppContext, exec: bool) -> Result<i32, anyhow::Error> {
    let config = load_config(&ctx.config_path, &ctx.root)?;
    
    let ssh_cmd = format!(
        "ssh -p {} {}@localhost -i {}",
        config.network.ssh_host_port,
        config.vscode.ssh_user,
        config.vscode.identity_file.to_string_lossy()
    );
    
    if exec {
        let mut cmd = Command::new("ssh");
        cmd.arg("-p").arg(config.network.ssh_host_port.to_string());
        cmd.arg(format!("{}@localhost", config.vscode.ssh_user));
        cmd.arg("-i").arg(&config.vscode.identity_file);
        cmd.status()?;
        Ok(0)
    } else {
        println!("{}", ssh_cmd);
        Ok(0)
    }
}
