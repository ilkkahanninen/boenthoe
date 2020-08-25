pub struct Envelope {
  pub duration: f32,
  pub get_value: Box<dyn Fn(f32) -> f32>,
}

impl Envelope {
  pub fn hold(duration: f32, value: f32) -> Envelope {
    Envelope {
      duration,
      get_value: Box::new(move |_| value),
    }
  }

  pub fn linear(duration: f32, from: f32, to: f32) -> Envelope {
    Envelope {
      duration,
      get_value: Box::new(move |pos| {
        let t = pos / duration;
        t * to + (1.0 - t) * from
      }),
    }
  }

  pub fn catmull(duration: f32, p0: f32, p1: f32, p2: f32, p3: f32) -> Envelope {
    Envelope {
      duration,
      get_value: Box::new(move |pos| {
        let t = pos / duration;
        let t2 = t * t;
        let t3 = t2 * t;
        0.5
          * ((2.0 * p1)
            + (-p0 + p2) * t
            + (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3) * t2
            + (-p0 + 3.0 * p1 - 3.0 * p2 + p3) * t3)
      }),
    }
  }

  pub fn concat(envelopes: Vec<Envelope>) -> Envelope {
    Envelope {
      duration: envelopes.iter().map(|e| e.duration).sum(),
      get_value: Box::new(move |pos| {
        let mut t = 0.0;
        let mut last_dur = 0.0;
        let env = envelopes.iter().find(|&env| {
          last_dur = env.duration;
          t += env.duration;
          t > pos
        });

        let sub_pos = pos - t + last_dur;

        match env {
          Some(e) => e.get_value(sub_pos),
          None => match envelopes.last() {
            Some(e) => e.get_value(sub_pos),
            None => 0.0,
          },
        }
      }),
    }
  }

  pub fn repeat(count: usize, envelope: Envelope) -> Envelope {
    Envelope {
      duration: envelope.duration * (count as f32),
      get_value: Box::new(move |t| {
        let dt = t % envelope.duration;
        envelope.get_value(dt)
      }),
    }
  }

  pub fn clip(duration: f32, envelope: Envelope) -> Envelope {
    Envelope {
      duration,
      get_value: Box::new(move |t| envelope.get_value(t.min(duration))),
    }
  }

  pub fn get_value(&self, t: f32) -> f32 {
    (self.get_value)(t.min(self.duration))
  }

  pub fn debug(&self, t: f32) {
    println!(
      "duration = {}, value at {} = {}",
      self.duration,
      t,
      self.get_value(t)
    );
  }
}
