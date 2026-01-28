use crate::cli::{AppContext, VscodeSubcommand};
use crate::config::load::load_config;
use crate::vscode::ssh_config::{print_ssh_config, install_ssh_config, remove_ssh_config};

pub fn handle_vscode(ctx: &AppContext, subcmd: VscodeSubcommand) -> Result<i32, anyhow::Error> {
    let config = load_config(&ctx.config_path, &ctx.root)?;
    
    match subcmd {
        VscodeSubcommand::Print => {
            println!("{}", print_ssh_config(&config));
            Ok(0)
        }
        VscodeSubcommand::Install => {
            install_ssh_config(&config)?;
            println!("VS Code SSH config installed");
            Ok(0)
        }
        VscodeSubcommand::Remove => {
            remove_ssh_config(&config)?;
            println!("VS Code SSH config removed");
            Ok(0)
        }
    }
}
