use crate::cli::{AppContext, AutostartSubcommand};
use crate::config::load::load_config;
use crate::autostart::{enable_autostart, disable_autostart, is_autostart_enabled};

pub fn handle_enable(ctx: &AppContext) -> Result<i32, anyhow::Error> {
    let config = load_config(&ctx.config_path, &ctx.root)?;
    enable_autostart(&config.vm.name, &ctx.root)?;
    println!("Autostart enabled");
    Ok(0)
}

pub fn handle_disable(ctx: &AppContext) -> Result<i32, anyhow::Error> {
    let config = load_config(&ctx.config_path, &ctx.root)?;
    disable_autostart(&config.vm.name)?;
    println!("Autostart disabled");
    Ok(0)
}

pub fn handle_autostart(ctx: &AppContext, subcmd: AutostartSubcommand) -> Result<i32, anyhow::Error> {
    let config = load_config(&ctx.config_path, &ctx.root)?;
    
    match subcmd {
        AutostartSubcommand::Status => {
            let enabled = is_autostart_enabled(&config.vm.name);
            match ctx.output_mode {
                crate::output::OutputMode::Json => {
                    use serde_json::json;
                    println!("{}", serde_json::to_string_pretty(&json!({
                        "enabled": enabled
                    }))?);
                }
                crate::output::OutputMode::Human => {
                    println!("Autostart: {}", if enabled { "enabled" } else { "disabled" });
                }
            }
            Ok(0)
        }
    }
}
