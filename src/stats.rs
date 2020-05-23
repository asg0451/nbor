use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Stats {
    last_start: Instant,
    last_dur: Duration,
    calls: u64,
    durs: Vec<Duration>,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            calls: 0,
            last_dur: Duration::default(),
            last_start: Instant::now(),
            durs: Vec::new(),
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

    pub fn print(&mut self) -> String {
        if self.durs.is_empty() {
            return "".to_string();
        }
        let avg = Duration::from_nanos(
            (self.durs.iter().sum::<Duration>().as_nanos() / (self.durs.len() as u128)) as u64,
        );
        self.durs.sort();
        let mid = self.durs.len() / 2;
        let med = self.durs[mid];

        format!(
            "last tick took {:?}; avg: {:?}, med: {:?}",
            self.last_dur, avg, med
        )
    }

    // making this the respoonsibility of someone else ie renderer to call so as
    // not to interfere with the simulator
    pub fn save_last_dur(&mut self) {
        if self.last_dur > Duration::default() {
            self.durs.push(self.last_dur);
        }
    }

    pub fn dump(&mut self, path: &std::path::Path) -> Result<(), std::io::Error> {
        use std::fs::File;
        use std::io::prelude::*;

        let durs = self
            .durs
            .iter()
            .map(|v| v.as_nanos().to_string())
            .collect::<Vec<String>>()
            .join("\n");

        let mut fh = File::create(&path)?;
        fh.write_all(durs.as_bytes())?;
        Ok(())
    }
}
