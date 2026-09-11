pub mod animation;
pub mod autostart;
pub mod cli;
pub mod hardware;
pub mod hid;
pub mod protocol;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::{Duration, Instant};

use clap::Parser;
use colored::Colorize;

use animation::{AnimationMode, DisplayAnimator};
use autostart::get_autostart_provider;
use cli::{AutostartAction, Cli, Commands};
use hardware::create_hardware_provider;
use hid::HidDeviceManager;
use protocol::{build_frequency_frame, build_temperature_frame, build_usage_frame};

// Catppuccin Mocha Color Palette Helpers
fn cat_mauve(s: &str) -> colored::ColoredString {
    s.truecolor(203, 166, 247)
}

fn cat_teal(s: &str) -> colored::ColoredString {
    s.truecolor(148, 226, 213)
}

fn cat_peach(s: &str) -> colored::ColoredString {
    s.truecolor(250, 179, 135)
}

fn cat_green(s: &str) -> colored::ColoredString {
    s.truecolor(166, 227, 161)
}

fn cat_red(s: &str) -> colored::ColoredString {
    s.truecolor(243, 139, 168)
}

fn cat_lavender(s: &str) -> colored::ColoredString {
    s.truecolor(180, 190, 254)
}

fn cat_subtext(s: &str) -> colored::ColoredString {
    s.truecolor(166, 173, 200)
}

fn print_banner() {
    println!(
        "{}",
        cat_mauve("  ____            _    ____             _ _             ")
            .bold()
    );
    println!(
        "{}",
        cat_mauve(" |  _ \\ _   _ ___| |_ / ___|___   ___  | (_)_ __   __ _ ")
            .bold()
    );
    println!(
        "{}",
        cat_lavender(" | |_) | | | / __| __| |   / _ \\ / _ \\ | | | '_ \\ / _` |")
            .bold()
    );
    println!(
        "{}",
        cat_lavender(" |  _ <| |_| \\__ \\ |_| |__| (_) | (_) || | | | | | (_| |")
            .bold()
    );
    println!(
        "{}",
        cat_teal(" |_| \\_\\\\__,_|___/\\__|\\____\\___/ \\___/ |_|_|_| |_|\\__, |")
            .bold()
    );
    println!(
        "{}",
        cat_teal("                                                  |___/ ")
            .bold()
    );
    println!(
        "  {} {} {}",
        cat_peach("RustCooling").bold(),
        cat_lavender("v1.0.0").bold(),
        cat_subtext("— ID-COOLING FX LCD Controller (<3 MB RAM)")
    );
    println!();
}

fn handle_status() {
    println!("{}", cat_mauve("=== RustCooling System Status ===").bold());

    // 1. Hardware sensors check
    let mut hw = create_hardware_provider();
    let temp = hw.get_cpu_temp();
    let load = hw.get_cpu_load();
    let freq = hw.get_cpu_freq();

    println!("{}", cat_lavender("\n[Sensors]").bold());
    if let Some(t) = temp {
        println!("  CPU Temperature : {}", cat_green(&format!("{:.1} °C", t)));
    } else {
        println!("  CPU Temperature : {}", cat_red("Unavailable / N/A"));
    }

    if let Some(l) = load {
        println!("  CPU Load        : {}", cat_green(&format!("{:.1} %", l)));
    } else {
        println!("  CPU Load        : {}", cat_red("Unavailable / N/A"));
    }

    if let Some(f) = freq {
        println!("  CPU Frequency   : {}", cat_green(&format!("{:.0} MHz", f)));
    } else {
        println!("  CPU Frequency   : {}", cat_red("Unavailable / N/A"));
    }

    // 2. HID Device check
    println!("{}", cat_lavender("\n[Device]").bold());
    let mut hid = HidDeviceManager::new();
    match hid.open_device() {
        Ok(()) => {
            println!(
                "  ID-COOLING LCD  : {} {}",
                cat_green("Connected").bold(),
                cat_subtext("(VID: 0x3402, PID: 0x0100)")
            );
        }
        Err(e) => {
            println!(
                "  ID-COOLING LCD  : {} ({})",
                cat_red("Disconnected").bold(),
                cat_subtext(&e)
            );
        }
    }

    // 3. Autostart status check
    println!("{}", cat_lavender("\n[Autostart]").bold());
    let autostart = get_autostart_provider();
    match autostart.is_enabled() {
        Ok(true) => {
            println!("  Service Status  : {}", cat_green("Enabled").bold());
        }
        Ok(false) => {
            println!("  Service Status  : {}", cat_subtext("Disabled"));
        }
        Err(e) => {
            println!("  Service Status  : {} ({})", cat_red("Error"), e);
        }
    }
    println!();
}

fn handle_autostart(action: AutostartAction) {
    let provider = get_autostart_provider();
    match action {
        AutostartAction::Enable => match provider.enable() {
            Ok(()) => {
                println!(
                    "{} RustCooling autostart enabled successfully.",
                    cat_green("✔").bold()
                );
            }
            Err(e) => {
                eprintln!(
                    "{} Failed to enable autostart: {}",
                    cat_red("✖").bold(),
                    e
                );
                std::process::exit(1);
            }
        },
        AutostartAction::Disable => match provider.disable() {
            Ok(()) => {
                println!(
                    "{} RustCooling autostart disabled successfully.",
                    cat_green("✔").bold()
                );
            }
            Err(e) => {
                eprintln!(
                    "{} Failed to disable autostart: {}",
                    cat_red("✖").bold(),
                    e
                );
                std::process::exit(1);
            }
        },
        AutostartAction::Status => match provider.is_enabled() {
            Ok(enabled) => {
                if enabled {
                    println!(
                        "RustCooling autostart is currently: {}",
                        cat_green("ENABLED").bold()
                    );
                } else {
                    println!(
                        "RustCooling autostart is currently: {}",
                        cat_subtext("DISABLED").bold()
                    );
                }
            }
            Err(e) => {
                eprintln!(
                    "{} Failed to query autostart status: {}",
                    cat_red("✖").bold(),
                    e
                );
                std::process::exit(1);
            }
        },
    }
}

fn run_monitor(interval_ms: u64, anim_mode: AnimationMode, anim_duration_ms: u64, daemon: bool) {
    if !daemon {
        print_banner();
        println!(
            "{} Starting LCD monitor loop (interval: {} ms, anim: {:?}, anim_duration: {} ms)...",
            cat_teal("▶").bold(),
            interval_ms,
            anim_mode,
            anim_duration_ms
        );
        println!("{}", cat_subtext("Press Ctrl+C to terminate cleanly.\n"));
    }

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    let _ = ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    });

    let mut hw = create_hardware_provider();
    let mut hid = HidDeviceManager::new();
    let mut animator = DisplayAnimator::new(anim_mode, anim_duration_ms);

    let tick_interval = Duration::from_millis(20);
    let poll_interval = Duration::from_millis(interval_ms);

    let mut last_poll = Instant::now() - poll_interval;
    let mut last_reconnect_attempt = Instant::now() - Duration::from_secs(10);

    let mut current_temp_display: u16 = 0;
    let mut current_load_display: u16 = 0;
    let mut current_freq_display: u16 = 0;

    while running.load(Ordering::SeqCst) {
        // Reconnect if needed
        if !hid.is_connected() && last_reconnect_attempt.elapsed() >= Duration::from_secs(2) {
            last_reconnect_attempt = Instant::now();
            let _ = hid.open_device();
        }

        // Poll sensor hardware on poll_interval
        if last_poll.elapsed() >= poll_interval {
            last_poll = Instant::now();

            let temp = hw.get_cpu_temp();
            let load = hw.get_cpu_load();
            let freq = hw.get_cpu_freq();

            if let Some(t) = temp {
                let target = t.round().clamp(0.0, 999.0) as u16;
                animator.set_target(target);
            }

            if let Some(l) = load {
                let load_u16 = l.round().clamp(0.0, 100.0) as u16;
                current_load_display = load_u16;
                if hid.is_connected() {
                    let _ = hid.send_raw(&build_usage_frame(load_u16));
                }
            }

            if let Some(f) = freq {
                let freq_u16 = f.round().clamp(0.0, 9999.0) as u16;
                current_freq_display = freq_u16;
                if hid.is_connected() {
                    let _ = hid.send_raw(&build_frequency_frame(freq_u16));
                }
            }
        }

        // Animation tick
        if let Some(animated_temp) = animator.tick() {
            current_temp_display = animated_temp;
            if hid.is_connected() {
                let _ = hid.send_raw(&build_temperature_frame(animated_temp));
            }
        }

        // Interactive status line
        if !daemon {
            let conn_str = if hid.is_connected() {
                cat_green("Connected")
            } else {
                cat_red("Searching...")
            };

            print!(
                "\r  {} Temp: {} | Load: {} | Freq: {} | Device: {}  ",
                cat_mauve("●"),
                cat_peach(&format!("{:3}°C", current_temp_display)).bold(),
                cat_teal(&format!("{:3}%", current_load_display)),
                cat_lavender(&format!("{:4} MHz", current_freq_display)),
                conn_str
            );
            use std::io::Write;
            let _ = std::io::stdout().flush();
        }

        sleep(tick_interval);
    }

    if !daemon {
        println!("\n\n{} Exiting gracefully...", cat_teal("ℹ"));
    }
    hid.close();
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Status) => {
            handle_status();
        }
        Some(Commands::Autostart { action }) => {
            handle_autostart(action);
        }
        Some(Commands::Run { interval }) => {
            let eff_interval = interval.unwrap_or(cli.interval);
            run_monitor(
                eff_interval,
                cli.animation.into(),
                cli.anim_duration,
                cli.daemon,
            );
        }
        None => {
            run_monitor(
                cli.interval,
                cli.animation.into(),
                cli.anim_duration,
                cli.daemon,
            );
        }
    }
}
