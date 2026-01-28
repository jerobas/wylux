pub mod commands;

use clap::{Parser, Subcommand};
use crate::config::paths::get_root;
use crate::output::OutputMode;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "portaqemu")]
#[command(about = "Portable QEMU VM manager for Windows")]
pub struct Cli {
    #[arg(long, global = true)]
    pub root: Option<PathBuf>,
    
    #[arg(long, global = true, default_value = "human")]
    pub output: String,
    
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize PortaQEMU directory structure
    Init,
    
    /// Start the VM
    Up {
        /// Attach to VM console
        #[arg(long)]
        attach: bool,
        /// Don't wait for SSH readiness
        #[arg(long)]
        no_wait: bool,
    },
    
    /// Stop the VM
    Down,
    
    /// Show VM status
    Status,
    
    /// SSH commands
    Ssh {
        /// Execute SSH command
        #[arg(long)]
        exec: bool,
    },
    
    /// Terminal fragment management
    Terminal {
        #[command(subcommand)]
        subcmd: TerminalSubcommand,
    },
    
    /// VS Code SSH config management
    Vscode {
        #[command(subcommand)]
        subcmd: VscodeSubcommand,
    },
    
    /// Autostart management
    Enable,
    
    /// Disable autostart
    Disable,
    
    /// Show autostart status
    Autostart {
        #[command(subcommand)]
        subcmd: AutostartSubcommand,
    },
    
    /// Run diagnostics
    Doctor,
}

#[derive(Subcommand)]
pub enum TerminalSubcommand {
    /// Install terminal fragment
    Install,
    /// Remove terminal fragment
    Remove,
    /// Show fragment path
    Path,
}

#[derive(Subcommand)]
pub enum VscodeSubcommand {
    /// Print SSH config block
    Print,
    /// Install SSH config block
    Install,
    /// Remove SSH config block
    Remove,
}

#[derive(Subcommand)]
pub enum AutostartSubcommand {
    /// Show autostart status
    Status,
}

pub struct AppContext {
    pub root: PathBuf,
    pub config_path: PathBuf,
    pub output_mode: OutputMode,
}

impl Cli {
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }
    
    pub fn output_mode(&self) -> OutputMode {
        match self.output.as_str() {
            "json" => OutputMode::Json,
            _ => OutputMode::Human,
        }
    }
}

pub fn run(cli: Cli) -> Result<i32, anyhow::Error> {
    let root = get_root(cli.root.as_ref());
    let config_path = root.join("config").join("portaqemu.toml");
    let output_mode = cli.output_mode();
    
    let ctx = AppContext {
        root,
        config_path,
        output_mode,
    };
    
    use Commands::*;
    match cli.command {
        Init => commands::handle_init(&ctx),
        Up { attach, no_wait } => commands::handle_up(&ctx, attach, no_wait),
        Down => commands::handle_down(&ctx),
        Status => commands::handle_status(&ctx),
        Ssh { exec } => commands::handle_ssh(&ctx, exec),
        Terminal { subcmd } => commands::handle_terminal(&ctx, subcmd),
        Vscode { subcmd } => commands::handle_vscode(&ctx, subcmd),
        Enable => commands::handle_enable(&ctx),
        Disable => commands::handle_disable(&ctx),
        Autostart { subcmd } => commands::handle_autostart(&ctx, subcmd),
        Doctor => commands::handle_doctor(&ctx),
    }
}
