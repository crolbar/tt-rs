use std::time::{Duration, Instant};
use anyhow::{Context, Result};


pub struct Timer {
    start_time: Option<Instant>,
    end_time: Option<Instant>,
    timer_time: Duration,
}

impl Timer {
    pub fn new(args: &Vec<String>) -> Result<Self> {
        let timer_time = {
            if let Some(time) = args.iter().position(|i| i == &"-t".to_string()) {
                Duration::from_secs(
                    args.get(time + 1)
                    .with_context(|| "add time after -t in secs (e.g: -t 30)")?
                    .parse()
                    .with_context(|| "incorrect duration: add time after -t in secs (e.g: -t 30)")?
                )
            } else {
                Duration::from_secs(1200) 
            }
        };


        Ok(
            Self {
                start_time: None,
                end_time: None,
                timer_time
            }
        )
    }

    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    pub fn stop(&mut self) {
        self.end_time = Some(Instant::now());
    }

    pub fn is_started(&self) -> bool {
        self.start_time.is_some()
    }

    pub fn is_stopped(&self) -> bool {
        self.start_time.is_some()
    }

    /// can fail if timer not started
    pub fn get_elapsed(&self) -> Duration {
        self.start_time.unwrap().elapsed()
    }

    pub fn get_remaining(&self) -> u64 {
        self.timer_time.as_secs().saturating_sub(
            self.get_elapsed().as_secs()
        )
    }

    pub fn is_out_of_time(&self) -> bool {
        if let Some(st) = self.start_time {
            return st.elapsed() >= self.timer_time
        }

        false
    }

    pub fn reset(&mut self) {
        self.start_time = None;
        self.end_time = None;
    }

    pub fn get_time(&self) -> Duration {
        if self.end_time.is_none() 
            && std::env::args().find(|a| a == "-t").is_some() 
        {
            return self.timer_time
        }

        self.end_time.unwrap() - self.start_time.unwrap()
    }
}
