use std::fmt::Debug;
use std::time::Instant;

pub trait TimeService: Debug{
    fn get_timestamp_ns(&self) -> u64;
}

#[derive(Debug)]
pub struct StdTimeService{
    start:Instant
}

impl Default for StdTimeService{
    fn default() -> Self {
        Self{
            start: Instant::now()
        }
    }
}

impl StdTimeService{
    pub fn new() -> Self{
        Default::default()
    }
}

impl TimeService for StdTimeService{
    fn get_timestamp_ns(&self) -> u64 {
        let elapsed = self.start.elapsed();
        elapsed.as_nanos().try_into().unwrap()
    }
}
