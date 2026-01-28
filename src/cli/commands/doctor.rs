use crate::cli::AppContext;
use crate::config::load::load_config;
use crate::doctor::checks::run_all_checks;
use crate::doctor::report::format_report;

pub fn handle_doctor(ctx: &AppContext) -> Result<i32, anyhow::Error> {
    let config = load_config(&ctx.config_path, &ctx.root)?;
    let checks = run_all_checks(&config, &ctx.root);
    
    println!("{}", format_report(&checks, ctx.output_mode));
    
    // Return non-zero if any checks failed
    let has_failures = checks.iter().any(|c| c.status == crate::doctor::checks::CheckStatus::Fail);
    Ok(if has_failures { 1 } else { 0 })
}
