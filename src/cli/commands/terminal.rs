use crate::cli::{AppContext, TerminalSubcommand};
use crate::config::load::load_config;
use crate::config::paths::get_fragment_file;
use crate::terminal::fragment::{install_fragment, remove_fragment};
use crate::output::OutputMode;

pub fn handle_terminal(ctx: &AppContext, subcmd: TerminalSubcommand) -> Result<i32, anyhow::Error> {
    let config = load_config(&ctx.config_path, &ctx.root)?;
    
    match subcmd {
        TerminalSubcommand::Install => {
            let fragment_path = install_fragment(&config)?;
            match ctx.output_mode {
                OutputMode::Json => {
                    use serde_json::json;
                    println!("{}", serde_json::to_string_pretty(&json!({
                        "fragment_file": fragment_path.to_string_lossy()
                    }))?);
                }
                OutputMode::Human => {
                    println!("Terminal fragment installed: {}", fragment_path.to_string_lossy());
                }
            }
            Ok(0)
        }
        TerminalSubcommand::Remove => {
            remove_fragment(&config.vm.name)?;
            match ctx.output_mode {
                OutputMode::Json => {
                    println!("{}", serde_json::to_string_pretty(&serde_json::json!({
                        "removed": true
                    }))?);
                }
                OutputMode::Human => {
                    println!("Terminal fragment removed");
                }
            }
            Ok(0)
        }
        TerminalSubcommand::Path => {
            let fragment_file = get_fragment_file(&config.vm.name);
            match ctx.output_mode {
                OutputMode::Json => {
                    use serde_json::json;
                    println!("{}", serde_json::to_string_pretty(&json!({
                        "fragment_root": fragment_file.parent().unwrap().to_string_lossy(),
                        "fragment_file": fragment_file.to_string_lossy()
                    }))?);
                }
                OutputMode::Human => {
                    println!("Fragment root: {}", fragment_file.parent().unwrap().to_string_lossy());
                    println!("Fragment file: {}", fragment_file.to_string_lossy());
                }
            }
            Ok(0)
        }
    }
}
