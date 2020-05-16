use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Stats {
    last_start: Instant,
    last_dur: Duration,
    calls: u64,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            calls: 0,
            last_dur: Duration::default(),
            last_start: Instant::now(),
        }
    }

    pub fn log_start(&mut self) {
        self.calls += 1;
        self.last_start = Instant::now();
    }

    pub fn log_end(&mut self) {
        self.calls += 1;
        self.last_dur = self.last_start.elapsed();
    }

    pub fn print(&self) -> String {
        format!("last tick took {:?}", self.last_dur)
    }
}
