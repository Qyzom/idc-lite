use std::fs;
use std::path::{Path, PathBuf};

use super::HardwareProvider;

pub struct LinuxHardwareProvider {
    prev_idle: u64,
    prev_total: u64,
    cached_temp_sensor: Option<PathBuf>,
}

impl LinuxHardwareProvider {
    pub fn new() -> Self {
        let mut provider = Self {
            prev_idle: 0,
            prev_total: 0,
            cached_temp_sensor: None,
        };
        let _ = provider.get_cpu_load();
        provider
    }

    fn find_temp_sensor() -> Option<PathBuf> {
        let hwmon_base = Path::new("/sys/class/hwmon");
        if !hwmon_base.exists() {
            return None;
        }

        let supported_drivers = ["k10temp", "coretemp", "zenpower", "cpu_thermal", "acpitz"];

        if let Ok(entries) = fs::read_dir(hwmon_base) {
            let mut candidates = Vec::new();

            for entry in entries.flatten() {
                let hwmon_dir = entry.path();
                let name_file = hwmon_dir.join("name");
                if let Ok(name) = fs::read_to_string(name_file) {
                    let name = name.trim();
                    for &driver in &supported_drivers {
                        if name.eq_ignore_ascii_case(driver) {
                            candidates.push(hwmon_dir.clone());
                            break;
                        }
                    }
                }
            }

            for hwmon_dir in candidates {
                if let Ok(files) = fs::read_dir(&hwmon_dir) {
                    let mut temp_inputs: Vec<PathBuf> = files
                        .flatten()
                        .filter_map(|e| {
                            let file_name = e.file_name();
                            let s = file_name.to_string_lossy();
                            if s.starts_with("temp") && s.ends_with("_input") {
                                Some(e.path())
                            } else {
                                None
                            }
                        })
                        .collect();

                    temp_inputs.sort();
                    if let Some(first_sensor) = temp_inputs.into_iter().next() {
                        return Some(first_sensor);
                    }
                }
            }
        }

        None
    }
}

impl Default for LinuxHardwareProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl HardwareProvider for LinuxHardwareProvider {
    fn get_cpu_temp(&mut self) -> Option<f32> {
        if self.cached_temp_sensor.is_none() {
            self.cached_temp_sensor = Self::find_temp_sensor();
        }

        if let Some(sensor_path) = &self.cached_temp_sensor {
            if let Ok(content) = fs::read_to_string(sensor_path) {
                if let Ok(val) = content.trim().parse::<f32>() {
                    return Some(val / 1000.0);
                }
            }
            // If reading failed, invalidate cache to retry discovery next time
            self.cached_temp_sensor = None;
        }

        None
    }

    fn get_cpu_load(&mut self) -> Option<f32> {
        let content = fs::read_to_string("/proc/stat").ok()?;
        let first_line = content.lines().next()?;
        if !first_line.starts_with("cpu ") {
            return None;
        }

        let parts: Vec<u64> = first_line
            .split_whitespace()
            .skip(1)
            .filter_map(|s| s.parse::<u64>().ok())
            .collect();

        if parts.len() < 4 {
            return None;
        }

        let user = parts[0];
        let nice = parts[1];
        let system = parts[2];
        let idle = parts[3];
        let iowait = parts.get(4).copied().unwrap_or(0);
        let irq = parts.get(5).copied().unwrap_or(0);
        let softirq = parts.get(6).copied().unwrap_or(0);
        let steal = parts.get(7).copied().unwrap_or(0);

        let idle_all = idle + iowait;
        let non_idle = user + nice + system + irq + softirq + steal;
        let total = idle_all + non_idle;

        let total_diff = total.saturating_sub(self.prev_total);
        let idle_diff = idle_all.saturating_sub(self.prev_idle);

        self.prev_total = total;
        self.prev_idle = idle_all;

        if total_diff == 0 {
            return Some(0.0);
        }

        let load = ((total_diff.saturating_sub(idle_diff)) as f32 / total_diff as f32) * 100.0;
        Some(load.clamp(0.0, 100.0))
    }

    fn get_cpu_freq(&mut self) -> Option<f32> {
        let scaling_cur_freq_path = Path::new("/sys/devices/system/cpu/cpu0/cpufreq/scaling_cur_freq");
        if scaling_cur_freq_path.exists() {
            if let Ok(content) = fs::read_to_string(scaling_cur_freq_path) {
                if let Ok(val) = content.trim().parse::<f32>() {
                    return Some(val / 1000.0);
                }
            }
        }

        if let Ok(content) = fs::read_to_string("/proc/cpuinfo") {
            for line in content.lines() {
                if line.starts_with("cpu MHz") {
                    if let Some((_, freq_str)) = line.split_once(':') {
                        if let Ok(freq) = freq_str.trim().parse::<f32>() {
                            return Some(freq);
                        }
                    }
                }
            }
        }

        None
    }
}
