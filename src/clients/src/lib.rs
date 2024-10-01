pub mod placement;
pub mod poll;

pub fn retry_times() -> usize {
    return 3;
}

pub fn retry_sleep_time(times: usize) -> u64 {
    return (times * 2) as u64;
}