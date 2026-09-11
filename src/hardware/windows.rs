use std::sync::Once;
use windows::core::{BSTR, HRESULT};
use windows::Win32::Foundation::FILETIME;
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CoSetProxyBlanket, CLSCTX_INPROC_SERVER,
    COINIT_MULTITHREADED, EOLE_AUTHENTICATION_CAPABILITIES, RPC_C_AUTHN_LEVEL_CALL,
    RPC_C_IMP_LEVEL_IMPERSONATE,
};
use windows::Win32::System::Threading::GetSystemTimes;
use windows::Win32::System::Wmi::{
    IWbemLocator, IWbemServices, WbemLocator, WBEM_FLAG_FORWARD_ONLY,
    WBEM_FLAG_RETURN_IMMEDIATELY, WBEM_INFINITE,
};

use super::HardwareProvider;

static INIT_COM: Once = Once::new();

fn init_com_once() {
    INIT_COM.call_once(|| unsafe {
        let _ = CoInitializeEx(None, COINIT_MULTITHREADED);
    });
}

fn filetime_to_u64(ft: &FILETIME) -> u64 {
    ((ft.dwHighDateTime as u64) << 32) | (ft.dwLowDateTime as u64)
}

struct WmiConnection {
    services: IWbemServices,
}

impl WmiConnection {
    fn connect(namespace: &str) -> Option<Self> {
        init_com_once();
        unsafe {
            let locator: IWbemLocator =
                CoCreateInstance(&WbemLocator, None, CLSCTX_INPROC_SERVER).ok()?;

            let ns_bstr = BSTR::from(namespace);
            let services = locator
                .ConnectServer(&ns_bstr, None, None, None, 0, None, None)
                .ok()?;

            // RPC_C_AUTHN_WINNT is 10, RPC_C_AUTHZ_NONE is 0
            let _ = CoSetProxyBlanket(
                &services,
                10, // RPC_C_AUTHN_WINNT
                0,  // RPC_C_AUTHZ_NONE
                None,
                RPC_C_AUTHN_LEVEL_CALL,
                RPC_C_IMP_LEVEL_IMPERSONATE,
                None,
                EOLE_AUTHENTICATION_CAPABILITIES(0),
            );

            Some(Self { services })
        }
    }

    fn query_single_f32(&self, query: &str, prop_name: &str) -> Option<f32> {
        unsafe {
            let q_bstr = BSTR::from(query);
            let lang_bstr = BSTR::from("WQL");
            let p_enumerator = self
                .services
                .ExecQuery(
                    &lang_bstr,
                    &q_bstr,
                    WBEM_FLAG_FORWARD_ONLY | WBEM_FLAG_RETURN_IMMEDIATELY,
                    None,
                )
                .ok()?;

            let mut class_objects = [None; 1];
            let mut returned = 0u32;

            let hr = p_enumerator.Next(
                WBEM_INFINITE,
                &mut class_objects,
                &mut returned,
            );

            if hr != HRESULT(0) || returned == 0 {
                return None;
            }

            let class_obj = class_objects[0].take()?;
            let prop_bstr = BSTR::from(prop_name);
            let mut val = windows::core::VARIANT::new();

            class_obj
                .Get(&prop_bstr, 0, &mut val, None, None)
                .ok()?;

            if let Ok(num) = f64::try_from(&val) {
                return Some(num as f32);
            }
            if let Ok(num) = u64::try_from(&val) {
                return Some(num as f32);
            }
            if let Ok(num) = i64::try_from(&val) {
                return Some(num as f32);
            }
            if let Ok(num) = u32::try_from(&val) {
                return Some(num as f32);
            }
            if let Ok(num) = i32::try_from(&val) {
                return Some(num as f32);
            }
            if let Ok(num) = u16::try_from(&val) {
                return Some(num as f32);
            }
            if let Ok(num) = i16::try_from(&val) {
                return Some(num as f32);
            }

            None
        }
    }
}

pub struct WindowsHardwareProvider {
    cimv2: Option<WmiConnection>,
    wmi_root: Option<WmiConnection>,
    prev_idle: u64,
    prev_kernel: u64,
    prev_user: u64,
}

impl WindowsHardwareProvider {
    pub fn new() -> Self {
        init_com_once();

        let cimv2 = WmiConnection::connect("root\\cimv2");
        let wmi_root = WmiConnection::connect("root\\wmi");

        let mut provider = Self {
            cimv2,
            wmi_root,
            prev_idle: 0,
            prev_kernel: 0,
            prev_user: 0,
        };

        // Initialize CPU load baseline
        let mut idle = FILETIME::default();
        let mut kernel = FILETIME::default();
        let mut user = FILETIME::default();

        unsafe {
            if GetSystemTimes(Some(&mut idle), Some(&mut kernel), Some(&mut user)).is_ok() {
                provider.prev_idle = filetime_to_u64(&idle);
                provider.prev_kernel = filetime_to_u64(&kernel);
                provider.prev_user = filetime_to_u64(&user);
            }
        }

        provider
    }
}

impl HardwareProvider for WindowsHardwareProvider {
    fn get_cpu_temp(&mut self) -> Option<f32> {
        // First try MSAcpi_ThermalZoneTemperature from root\wmi
        // CurrentTemperature is in tenths of Kelvin (0.1K), temp_c = (temp_k / 10.0) - 273.15
        if let Some(ref wmi) = self.wmi_root {
            if let Some(raw_k) = wmi.query_single_f32(
                "SELECT CurrentTemperature FROM MSAcpi_ThermalZoneTemperature",
                "CurrentTemperature",
            ) {
                if raw_k > 0.0 {
                    let temp_c = (raw_k / 10.0) - 273.15;
                    if (0.0..=120.0).contains(&temp_c) {
                        return Some(temp_c);
                    }
                }
            }
        }

        // Try Win32_ThermalZone / counters from root\cimv2
        if let Some(ref cimv2) = self.cimv2 {
            if let Some(raw_k) = cimv2.query_single_f32(
                "SELECT Temperature FROM Win32_PerfFormattedData_Counters_ThermalZoneInformation",
                "Temperature",
            ) {
                if raw_k > 0.0 {
                    let temp_c = raw_k - 273.15;
                    if (0.0..=120.0).contains(&temp_c) {
                        return Some(temp_c);
                    }
                }
            }

            if let Some(raw_k) = cimv2.query_single_f32(
                "SELECT CurrentTemperature FROM Win32_ThermalZone",
                "CurrentTemperature",
            ) {
                if raw_k > 0.0 {
                    let temp_c = (raw_k / 10.0) - 273.15;
                    if (0.0..=120.0).contains(&temp_c) {
                        return Some(temp_c);
                    }
                }
            }

            if let Some(temp_c) = cimv2.query_single_f32(
                "SELECT CurrentReading FROM Win32_TemperatureProbe",
                "CurrentReading",
            ) {
                if (0.0..=120.0).contains(&temp_c) {
                    return Some(temp_c);
                }
            }
        }

        None
    }

    fn get_cpu_load(&mut self) -> Option<f32> {
        let mut idle = FILETIME::default();
        let mut kernel = FILETIME::default();
        let mut user = FILETIME::default();

        unsafe {
            if GetSystemTimes(Some(&mut idle), Some(&mut kernel), Some(&mut user)).is_ok() {
                let current_idle = filetime_to_u64(&idle);
                let current_kernel = filetime_to_u64(&kernel);
                let current_user = filetime_to_u64(&user);

                let diff_idle = current_idle.saturating_sub(self.prev_idle);
                let diff_kernel = current_kernel.saturating_sub(self.prev_kernel);
                let diff_user = current_user.saturating_sub(self.prev_user);

                self.prev_idle = current_idle;
                self.prev_kernel = current_kernel;
                self.prev_user = current_user;

                let total_sys = diff_kernel + diff_user;
                if total_sys > 0 {
                    let total_busy = total_sys.saturating_sub(diff_idle);
                    let load = (total_busy as f32 / total_sys as f32) * 100.0;
                    return Some(load.clamp(0.0, 100.0));
                }
            }
        }

        // Fallback: WMI PercentProcessorTime from Win32_PerfFormattedData_PerfOS_Processor
        if let Some(ref cimv2) = self.cimv2 {
            if let Some(load) = cimv2.query_single_f32(
                "SELECT PercentProcessorTime FROM Win32_PerfFormattedData_PerfOS_Processor WHERE Name='_Total'",
                "PercentProcessorTime",
            ) {
                return Some(load.clamp(0.0, 100.0));
            }
        }

        None
    }

    fn get_cpu_freq(&mut self) -> Option<f32> {
        if let Some(ref cimv2) = self.cimv2 {
            if let Some(mhz) = cimv2.query_single_f32(
                "SELECT CurrentClockSpeed FROM Win32_Processor",
                "CurrentClockSpeed",
            ) {
                if mhz > 0.0 {
                    return Some(mhz);
                }
            }
        }

        None
    }
}
