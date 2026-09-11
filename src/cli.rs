use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(
    name = "rustcooling",
    author = "Qyzom <t0lotsmail@gmail.com>",
    version = "1.0.0",
    about = "High-performance, ultra-lightweight LCD CLI & daemon for ID-COOLING FX coolers (<3 MB RAM)",
    long_about = "RustCooling is the next-generation, high-performance successor to idc-lite.\nCrafted in 100% pure modern Rust with zero bloated runtimes, zero closed-source drivers,\nand an astonishingly low memory footprint (~2 MB RAM)."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Display update interval in milliseconds (default: 1000)
    #[arg(short, long, default_value_t = 1000)]
    pub interval: u64,

    /// Animation mode for display values
    #[arg(short, long, value_enum, default_value_t = AnimationCliMode::Smooth)]
    pub animation: AnimationCliMode,

    /// Animation duration in milliseconds (default: 300)
    #[arg(short = 'd', long, default_value_t = 300)]
    pub anim_duration: u64,

    /// Run as background daemon (suppress interactive terminal output)
    #[arg(long)]
    pub daemon: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run the display monitor loop (default when no subcommand is given)
    Run {
        /// Override display update interval (ms)
        #[arg(short, long)]
        interval: Option<u64>,
    },
    /// Inspect connected ID-COOLING LCD hardware and sensors
    Status,
    /// Manage system autostart service (Windows Task Scheduler / Linux systemd)
    Autostart {
        #[command(subcommand)]
        action: AutostartAction,
    },
}

#[derive(Subcommand, Debug)]
pub enum AutostartAction {
    /// Register RustCooling in system autostart
    Enable,
    /// Remove RustCooling from system autostart
    Disable,
    /// Check current autostart status
    Status,
}

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnimationCliMode {
    None,
    Smooth,
    Roller,
}

impl From<AnimationCliMode> for crate::animation::AnimationMode {
    fn from(mode: AnimationCliMode) -> Self {
        match mode {
            AnimationCliMode::None => crate::animation::AnimationMode::None,
            AnimationCliMode::Smooth => crate::animation::AnimationMode::Smooth,
            AnimationCliMode::Roller => crate::animation::AnimationMode::Roller,
        }
    }
}
