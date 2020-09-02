#[macro_export]
macro_rules! create_state {
  ($T:ident { $($k:ident => $e:expr),* }) => {
    {
      Box::new(|pos: &f64| $T {
        $(
          $k: $e.get_value(pos),
        )*
      })
    }
  };
}

pub struct Envelope {
    pub duration: f64,
    pub get_value: Box<dyn Fn(&f64) -> f64>,
}

#[allow(dead_code)]
impl Envelope {
    pub fn hold(duration: f64, value: f64) -> Self {
        Self {
            duration,
            get_value: Box::new(move |_| value),
        }
    }

    pub fn linear(duration: f64, from: f64, to: f64) -> Self {
        Self {
            duration,
            get_value: Box::new(move |pos| {
                let t = pos / duration;
                t * to + (1.0 - t) * from
            }),
        }
    }

    pub fn catmull(duration: f64, p0: f64, p1: f64, p2: f64, p3: f64) -> Self {
        Self {
            duration,
            get_value: Box::new(move |pos| {
                let t = pos / duration;
                let t2 = t * t;
                let t3 = t2 * t;
                0.5 * ((2.0 * p1)
                    + (-p0 + p2) * t
                    + (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3) * t2
                    + (-p0 + 3.0 * p1 - 3.0 * p2 + p3) * t3)
            }),
        }
    }

    pub fn concat(envelopes: Vec<Envelope>) -> Self {
        Self {
            duration: envelopes.iter().map(|e| e.duration).sum(),
            get_value: Box::new(move |pos| {
                let mut t = 0.0;
                let mut last_dur = 0.0;
                let env = envelopes.iter().find(|&env| {
                    last_dur = env.duration;
                    t += env.duration;
                    &t > &pos
                });

                let sub_pos = pos - t + last_dur;

                match env {
                    Some(e) => e.get_value(&sub_pos),
                    None => match envelopes.last() {
                        Some(e) => e.get_value(&sub_pos),
                        None => 0.0,
                    },
                }
            }),
        }
    }

    pub fn repeat(count: usize, envelope: Envelope) -> Self {
        Self {
            duration: envelope.duration * (count as f64),
            get_value: Box::new(move |t| {
                let dt = t % envelope.duration;
                envelope.get_value(&dt)
            }),
        }
    }

    pub fn infinite(envelope: Envelope) -> Self {
        Self {
            duration: std::f64::INFINITY,
            get_value: Box::new(move |t| {
                let dt = t % envelope.duration;
                envelope.get_value(&dt)
            }),
        }
    }

    pub fn clip(duration: f64, envelope: Envelope) -> Self {
        Self {
            duration,
            get_value: Box::new(move |t| envelope.get_value(&t.min(duration))),
        }
    }

    pub fn get_value(&self, t: &f64) -> f64 {
        (self.get_value)(&t.min(self.duration))
    }

    pub fn debug(&self, t: f64) {
        println!(
            "duration = {}, value at {} = {}",
            self.duration,
            t,
            self.get_value(&t)
        );
    }
}
