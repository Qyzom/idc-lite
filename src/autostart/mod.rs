#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "windows")]
pub mod windows;

pub trait AutostartProvider {
    fn enable(&self) -> Result<(), String>;
    fn disable(&self) -> Result<(), String>;
    fn is_enabled(&self) -> Result<bool, String>;
}

pub fn get_autostart_provider() -> Box<dyn AutostartProvider> {
    #[cfg(target_os = "windows")]
    {
        Box::new(windows::WindowsAutostart::new())
    }
    #[cfg(target_os = "linux")]
    {
        Box::new(linux::LinuxAutostart::new())
    }
    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    {
        struct UnsupportedAutostart;
        impl AutostartProvider for UnsupportedAutostart {
            fn enable(&self) -> Result<(), String> { Err("Autostart not supported on this OS".into()) }
            fn disable(&self) -> Result<(), String> { Err("Autostart not supported on this OS".into()) }
            fn is_enabled(&self) -> Result<bool, String> { Ok(false) }
        }
        Box::new(UnsupportedAutostart)
    }
}
