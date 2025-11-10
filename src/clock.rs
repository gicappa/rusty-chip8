use std::thread::sleep;
use std::time::{Duration, Instant};

pub const INTERVAL: Duration = Duration::from_micros(16_666);

pub struct Clock {
    start_time: Instant,
    end_time: Instant,
}

impl Clock {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            end_time: Instant::now(),
        }
    }

    pub fn start(&mut self) {
        self.start_time = Instant::now()
    }

    pub fn stop_and_wait(&mut self) {
        self.end_time = Instant::now();
        let elapsed = self.end_time - self.start_time;
        if elapsed < INTERVAL {
            sleep(INTERVAL - elapsed);
            self.end_time += INTERVAL;
        } else {
            self.end_time = self.start_time;
        }
    }
}
