use std::time::Instant;

pub struct Timer {
  start_time: Instant,
}

impl Timer {
  pub fn new() -> Self {
    Self {
      start_time: Instant::now(),
    }
  }

  pub fn reset(&mut self) {
    self.start_time = Instant::now();
  }

  pub fn elapsed(&self) -> f64 {
    self.start_time.elapsed().as_millis() as f64 * 0.001
  }
}
