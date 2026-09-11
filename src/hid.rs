use hidapi::{HidApi, HidDevice};

pub const VENDOR_ID: u16 = 0x3402;
pub const PRODUCT_ID: u16 = 0x0100;

pub struct HidDeviceManager {
    device: Option<HidDevice>,
}

impl HidDeviceManager {
    pub fn new() -> Self {
        Self { device: None }
    }

    pub fn open_device(&mut self) -> Result<(), String> {
        let api = HidApi::new().map_err(|e| format!("Failed to initialize HidApi: {e}"))?;

        for device_info in api.device_list() {
            if device_info.vendor_id() == VENDOR_ID && device_info.product_id() == PRODUCT_ID {
                match device_info.open_device(&api) {
                    Ok(dev) => {
                        self.device = Some(dev);
                        return Ok(());
                    }
                    Err(e) => {
                        return Err(format!(
                            "Failed to open device (VID: 0x{:04X}, PID: 0x{:04X}): {e}",
                            VENDOR_ID, PRODUCT_ID
                        ));
                    }
                }
            }
        }

        Err(format!(
            "Device not found (VID: 0x{:04X}, PID: 0x{:04X})",
            VENDOR_ID, PRODUCT_ID
        ))
    }

    pub fn is_connected(&self) -> bool {
        self.device.is_some()
    }

    pub fn send_raw(&mut self, frame: &[u8; crate::protocol::REPORT_SIZE]) -> Result<(), String> {
        match self.device.as_mut() {
            Some(dev) => {
                match dev.write(frame) {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        self.device = None;
                        Err(format!("HID write error: {e}"))
                    }
                }
            }
            None => Err("Device is not opened".to_string()),
        }
    }

    pub fn close(&mut self) {
        self.device = None;
    }
}

impl Default for HidDeviceManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manager_initial_state() {
        let manager = HidDeviceManager::new();
        assert!(!manager.is_connected());
    }

    #[test]
    fn test_manager_close() {
        let mut manager = HidDeviceManager::new();
        manager.close();
        assert!(!manager.is_connected());
    }

    #[test]
    fn test_send_raw_disconnected() {
        let mut manager = HidDeviceManager::new();
        let frame = [0u8; crate::protocol::REPORT_SIZE];
        let result = manager.send_raw(&frame);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Device is not opened");
    }
}
