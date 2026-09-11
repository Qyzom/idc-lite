#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "windows")]
pub mod windows;

pub trait HardwareProvider {
    fn get_cpu_temp(&mut self) -> Option<f32>;
    fn get_cpu_load(&mut self) -> Option<f32>;
    fn get_cpu_freq(&mut self) -> Option<f32>;
}

pub struct DummyProvider;

impl DummyProvider {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DummyProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl HardwareProvider for DummyProvider {
    fn get_cpu_temp(&mut self) -> Option<f32> {
        None
    }

    fn get_cpu_load(&mut self) -> Option<f32> {
        None
    }

    fn get_cpu_freq(&mut self) -> Option<f32> {
        None
    }
}

pub fn create_hardware_provider() -> Box<dyn HardwareProvider> {
    #[cfg(target_os = "linux")]
    {
        Box::new(linux::LinuxHardwareProvider::new())
    }

    #[cfg(target_os = "windows")]
    {
        Box::new(windows::WindowsHardwareProvider::new())
    }

    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    {
        Box::new(DummyProvider::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dummy_provider() {
        let mut provider = DummyProvider::new();
        assert_eq!(provider.get_cpu_temp(), None);
        assert_eq!(provider.get_cpu_load(), None);
        assert_eq!(provider.get_cpu_freq(), None);
    }

    #[test]
    fn test_create_hardware_provider() {
        let mut provider = create_hardware_provider();
        // Provider shouldn't panic on call
        let _ = provider.get_cpu_temp();
        let _ = provider.get_cpu_load();
        let _ = provider.get_cpu_freq();
    }
}
