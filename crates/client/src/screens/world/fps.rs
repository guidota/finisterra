use std::time::{Duration, Instant};

pub struct Fps {
    last_frames: Vec<f32>,
    last_update: Instant,
}

impl Fps {
    const N: usize = 8;
    const REFRESH: u64 = 16;
    pub fn update(&mut self, delta: Duration) {
        let now = Instant::now();
        if now - self.last_update > Duration::from_millis(Self::REFRESH) {
            self.last_update = now;
            self.last_frames.push(1. / delta.as_secs_f32());
            if self.last_frames.len() > Self::N {
                self.last_frames.remove(0);
            }
        }
    }

    pub fn get(&self) -> f32 {
        self.last_frames.iter().sum::<f32>() / Self::N as f32
    }
}

impl Default for Fps {
    fn default() -> Self {
        Self {
            last_frames: vec![],
            last_update: Instant::now(),
        }
    }
}
