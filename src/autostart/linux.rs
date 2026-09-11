use super::AutostartProvider;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub struct LinuxAutostart;

impl LinuxAutostart {
    pub fn new() -> Self {
        Self
    }

    fn unit_path() -> Result<PathBuf, String> {
        let home = env::var("HOME").map_err(|_| "HOME environment variable not set".to_string())?;
        let mut path = PathBuf::from(home);
        path.push(".config/systemd/user");
        fs::create_dir_all(&path).map_err(|e| format!("Failed to create systemd user dir: {e}"))?;
        path.push("rustcooling.service");
        Ok(path)
    }
}

impl Default for LinuxAutostart {
    fn default() -> Self {
        Self::new()
    }
}

impl AutostartProvider for LinuxAutostart {
    fn enable(&self) -> Result<(), String> {
        let exe_path = env::current_exe().map_err(|e| format!("Failed to get exe path: {e}"))?;
        let unit_file = Self::unit_path()?;

        let content = format!(
            "[Unit]\nDescription=RustCooling ID-COOLING LCD Daemon\nAfter=default.target\n\n[Service]\nExecStart={} --daemon\nRestart=on-failure\nRestartSec=3\n\n[Install]\nWantedBy=default.target\n",
            exe_path.display()
        );

        fs::write(&unit_file, content).map_err(|e| format!("Failed to write service file: {e}"))?;

        let _ = Command::new("systemctl").args(["--user", "daemon-reload"]).status();
        let status = Command::new("systemctl")
            .args(["--user", "enable", "--now", "rustcooling.service"])
            .status()
            .map_err(|e| format!("Failed to enable systemd unit: {e}"))?;

        if status.success() {
            Ok(())
        } else {
            Err("systemctl enable returned non-zero code".into())
        }
    }

    fn disable(&self) -> Result<(), String> {
        let _ = Command::new("systemctl")
            .args(["--user", "disable", "--now", "rustcooling.service"])
            .status();

        if let Ok(path) = Self::unit_path() {
            if path.exists() {
                let _ = fs::remove_file(path);
            }
        }

        let _ = Command::new("systemctl").args(["--user", "daemon-reload"]).status();
        Ok(())
    }

    fn is_enabled(&self) -> Result<bool, String> {
        let output = Command::new("systemctl")
            .args(["--user", "is-enabled", "rustcooling.service"])
            .output()
            .map_err(|e| format!("Failed to check systemd unit: {e}"))?;

        let text = String::from_utf8_lossy(&output.stdout);
        Ok(text.trim() == "enabled")
    }
}
