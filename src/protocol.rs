pub const VID: u16 = 0x3402;
pub const PID: u16 = 0x0100;
pub const REPORT_SIZE: usize = 64;

pub const CMD_TEMPERATURE: u8 = 0x01;
pub const CMD_FREQUENCY: u8 = 0x02;
pub const CMD_USAGE: u8 = 0x03;
pub const CMD_SHOW: u8 = 0x04;

pub const MAGIC_HEADER: [u8; 4] = [0x55, 0xaa, 0x01, 0x00];

pub fn build_frame(command: u8, value: u16) -> [u8; REPORT_SIZE] {
    let mut frame = [0u8; REPORT_SIZE];
    frame[0..4].copy_from_slice(&MAGIC_HEADER);
    frame[4] = command;
    let bytes = value.to_be_bytes();
    frame[5] = bytes[0];
    frame[6] = bytes[1];
    frame[7] = command.wrapping_add(bytes[0]).wrapping_add(bytes[1]);
    frame
}

pub fn build_temperature_frame(temp: u16) -> [u8; REPORT_SIZE] {
    build_frame(CMD_TEMPERATURE, temp)
}

pub fn build_frequency_frame(freq: u16) -> [u8; REPORT_SIZE] {
    build_frame(CMD_FREQUENCY, freq)
}

pub fn build_usage_frame(usage: u16) -> [u8; REPORT_SIZE] {
    build_frame(CMD_USAGE, usage)
}

pub fn build_show_frame(on: bool) -> [u8; REPORT_SIZE] {
    build_frame(CMD_SHOW, if on { 1 } else { 0 })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temperature_frame_45c() {
        let frame = build_temperature_frame(45);
        assert_eq!(frame.len(), REPORT_SIZE);
        assert_eq!(frame.len(), 64);
        assert_eq!(
            &frame[0..8],
            &[0x55, 0xAA, 0x01, 0x00, 0x01, 0x00, 0x2D, 0x2E]
        );
        for &b in &frame[8..64] {
            assert_eq!(b, 0);
        }
    }

    #[test]
    fn test_checksum_and_length() {
        let commands = [
            (CMD_TEMPERATURE, 0x1234u16),
            (CMD_FREQUENCY, 4200u16),
            (CMD_USAGE, 99u16),
            (CMD_SHOW, 1u16),
        ];

        for (cmd, val) in commands {
            let frame = build_frame(cmd, val);
            assert_eq!(frame.len(), 64);
            let bytes = val.to_be_bytes();
            let expected_checksum = cmd.wrapping_add(bytes[0]).wrapping_add(bytes[1]);
            assert_eq!(frame[7], expected_checksum);
            assert_eq!(frame[4], cmd);
            assert_eq!(frame[5], bytes[0]);
            assert_eq!(frame[6], bytes[1]);
            assert_eq!(&frame[0..4], &MAGIC_HEADER);
            for &b in &frame[8..] {
                assert_eq!(b, 0);
            }
        }
    }

    #[test]
    fn test_helper_methods() {
        let freq_frame = build_frequency_frame(3500);
        assert_eq!(freq_frame[4], CMD_FREQUENCY);
        assert_eq!(freq_frame[5..7], 3500u16.to_be_bytes());

        let usage_frame = build_usage_frame(50);
        assert_eq!(usage_frame[4], CMD_USAGE);
        assert_eq!(usage_frame[5..7], 50u16.to_be_bytes());

        let show_on = build_show_frame(true);
        assert_eq!(show_on[4], CMD_SHOW);
        assert_eq!(show_on[5..7], 1u16.to_be_bytes());

        let show_off = build_show_frame(false);
        assert_eq!(show_off[4], CMD_SHOW);
        assert_eq!(show_off[5..7], 0u16.to_be_bytes());
    }
}
