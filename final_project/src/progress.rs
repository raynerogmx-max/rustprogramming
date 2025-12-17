use std::time::Duration;

#[derive(Default)]
pub struct ProgressTracker {
    pub processed: usize,
    pub errors: usize,
    pub total_time: Duration,
}

impl ProgressTracker {
    pub fn update(&mut self, time: Duration, had_error: bool) {
        self.processed += 1;
        self.total_time += time;
        if had_error {
            self.errors += 1;
        }
    }
}
