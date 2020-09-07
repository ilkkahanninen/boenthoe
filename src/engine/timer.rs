use std::time::Instant;

pub struct Timer {
    start_time: Instant,
    adjust: f64,
}

#[allow(dead_code)]
impl Timer {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            adjust: 0.0,
        }
    }

    pub fn reset(&mut self) {
        self.start_time = Instant::now();
    }

    pub fn elapsed(&self) -> f64 {
        self.true_elapsed() + self.adjust
    }

    pub fn true_elapsed(&self) -> f64 {
        self.start_time.elapsed().as_millis() as f64 * 0.001
    }

    pub fn forward(&mut self, seconds: f64) {
        self.adjust = (self.adjust + seconds).max(-self.true_elapsed())
    }
}
