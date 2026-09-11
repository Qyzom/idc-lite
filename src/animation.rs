use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationMode {
    None,
    Smooth,
    Roller,
}

pub struct DisplayAnimator {
    pub current_val: f64,
    pub target_val: u16,
    pub start_val: f64,
    pub start_time: Instant,
    pub duration: Duration,
    pub mode: AnimationMode,
    pub last_sent: u16,
}

impl DisplayAnimator {
    pub fn new(mode: AnimationMode, duration_ms: u64) -> Self {
        Self {
            current_val: 0.0,
            target_val: 0,
            start_val: 0.0,
            start_time: Instant::now(),
            duration: Duration::from_millis(duration_ms),
            mode,
            last_sent: 0,
        }
    }

    pub fn set_target(&mut self, target: u16) {
        if self.target_val == target {
            return;
        }
        self.start_val = self.current_val;
        self.target_val = target;
        self.start_time = Instant::now();
    }

    pub fn tick(&mut self) -> Option<u16> {
        let new_display_val = match self.mode {
            AnimationMode::None => {
                self.current_val = self.target_val as f64;
                self.target_val
            }
            AnimationMode::Smooth => {
                if self.duration.is_zero() {
                    self.current_val = self.target_val as f64;
                } else {
                    let elapsed = self.start_time.elapsed();
                    let t = (elapsed.as_secs_f64() / self.duration.as_secs_f64()).clamp(0.0, 1.0);
                    let progress = if t < 0.5 {
                        4.0 * t * t * t
                    } else {
                        1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
                    };
                    self.current_val = self.start_val + (self.target_val as f64 - self.start_val) * progress;
                    if t >= 1.0 {
                        self.current_val = self.target_val as f64;
                    }
                }
                self.current_val.round().clamp(0.0, u16::MAX as f64) as u16
            }
            AnimationMode::Roller => {
                let target_f = self.target_val as f64;
                let diff = (target_f - self.start_val).abs();

                if diff == 0.0 || self.duration.is_zero() {
                    self.current_val = target_f;
                } else {
                    // Calculate step interval so all diff steps are distributed over duration
                    let step_interval = self.duration.as_secs_f64() / diff;
                    let elapsed = self.start_time.elapsed().as_secs_f64();
                    let steps_taken = (elapsed / step_interval).floor().min(diff);

                    if self.target_val as f64 >= self.start_val {
                        self.current_val = self.start_val + steps_taken;
                    } else {
                        self.current_val = self.start_val - steps_taken;
                    }
                }
                self.current_val.round().clamp(0.0, u16::MAX as f64) as u16
            }
        };

        if new_display_val != self.last_sent {
            self.last_sent = new_display_val;
            Some(new_display_val)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;

    #[test]
    fn test_mode_none() {
        let mut anim = DisplayAnimator::new(AnimationMode::None, 500);
        assert_eq!(anim.last_sent, 0);

        anim.set_target(50);
        let tick_result = anim.tick();
        assert_eq!(tick_result, Some(50));
        assert_eq!(anim.last_sent, 50);

        // Subsequent ticks without change return None
        assert_eq!(anim.tick(), None);

        // Another target
        anim.set_target(100);
        assert_eq!(anim.tick(), Some(100));
        assert_eq!(anim.tick(), None);
    }

    #[test]
    fn test_mode_smooth_transition() {
        let mut anim = DisplayAnimator::new(AnimationMode::Smooth, 100);
        anim.set_target(100);

        let mut values = Vec::new();
        let start = Instant::now();
        while start.elapsed() < Duration::from_millis(150) {
            if let Some(val) = anim.tick() {
                values.push(val);
            }
            sleep(Duration::from_millis(10));
        }

        // Must reach the target
        assert_eq!(anim.tick(), None);
        assert_eq!(anim.last_sent, 100);
        assert!(!values.is_empty());
        assert_eq!(*values.last().unwrap(), 100);

        // Check values are monotonically increasing
        for w in values.windows(2) {
            assert!(w[0] <= w[1]);
        }
    }

    #[test]
    fn test_mode_roller_step_transition() {
        let mut anim = DisplayAnimator::new(AnimationMode::Roller, 100);
        anim.set_target(10);

        let mut values = Vec::new();
        let start = Instant::now();
        while start.elapsed() < Duration::from_millis(150) {
            if let Some(val) = anim.tick() {
                values.push(val);
            }
            sleep(Duration::from_millis(5));
        }

        assert_eq!(anim.last_sent, 10);
        assert!(!values.is_empty());
        assert_eq!(*values.last().unwrap(), 10);

        // Step by step down
        anim.set_target(5);
        let start = Instant::now();
        let mut down_values = Vec::new();
        while start.elapsed() < Duration::from_millis(150) {
            if let Some(val) = anim.tick() {
                down_values.push(val);
            }
            sleep(Duration::from_millis(5));
        }
        assert_eq!(anim.last_sent, 5);
        for w in down_values.windows(2) {
            assert!(w[0] >= w[1]);
        }
    }

    #[test]
    fn test_no_freeze_or_overflow() {
        // Zero duration
        let mut anim = DisplayAnimator::new(AnimationMode::Smooth, 0);
        anim.set_target(u16::MAX);
        assert_eq!(anim.tick(), Some(u16::MAX));
        assert_eq!(anim.tick(), None);

        // Roller with 0 duration
        let mut roller = DisplayAnimator::new(AnimationMode::Roller, 0);
        roller.set_target(u16::MAX);
        assert_eq!(roller.tick(), Some(u16::MAX));
        assert_eq!(roller.tick(), None);

        // Roller stepping to same target
        roller.set_target(u16::MAX);
        assert_eq!(roller.tick(), None);
    }
}
