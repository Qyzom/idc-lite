use super::AutostartProvider;
use std::env;
use std::process::Command;

pub struct WindowsAutostart {
    task_name: &'static str,
}

impl WindowsAutostart {
    pub fn new() -> Self {
        Self {
            task_name: "RustCoolingDaemon",
        }
    }
}

impl Default for WindowsAutostart {
    fn default() -> Self {
        Self::new()
    }
}

impl AutostartProvider for WindowsAutostart {
    fn enable(&self) -> Result<(), String> {
        let exe_path = env::current_exe().map_err(|e| format!("Failed to get exe path: {e}"))?;
        let exe_str = exe_path.to_str().ok_or("Invalid executable path UTF-8")?;

        let tr_arg = format!("\"{}\" --daemon", exe_str);

        // schtasks /Create /SC ONLOGON /TN RustCoolingDaemon /TR "path --daemon" /F
        let status = Command::new("schtasks")
            .args(["/Create", "/SC", "ONLOGON", "/TN", self.task_name, "/TR", &tr_arg, "/F"])
            .status()
            .map_err(|e| format!("Failed to execute schtasks: {e}"))?;

        if status.success() {
            Ok(())
        } else {
            Err(format!("schtasks failed with exit code: {:?}", status.code()))
        }
    }

    fn disable(&self) -> Result<(), String> {
        // schtasks /Delete /TN RustCoolingDaemon /F
        let status = Command::new("schtasks")
            .args(["/Delete", "/TN", self.task_name, "/F"])
            .status()
            .map_err(|e| format!("Failed to execute schtasks: {e}"))?;

        if status.success() {
            Ok(())
        } else {
            Err(format!("schtasks delete failed with exit code: {:?}", status.code()))
        }
    }

    fn is_enabled(&self) -> Result<bool, String> {
        let output = Command::new("schtasks")
            .args(["/Query", "/TN", self.task_name])
            .output()
            .map_err(|e| format!("Failed to query schtasks: {e}"))?;

        Ok(output.status.success())
    }
}
