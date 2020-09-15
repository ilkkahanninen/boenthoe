#[macro_export]
macro_rules! create_state {
    ($T:ident { $($k:ident => $e:expr),* }) => {{
        let f: StateFn<$T> = Box::new(move |time: &f32| $T {
            $(
                $k: $e.get_value(time),
            )*
        });
        f
    }};
}

pub type StateFn<T> = Box<dyn Fn(&f32) -> T>;

pub struct Envelope {
    pub duration: f32,
    pub get_value: Box<dyn Fn(&f32) -> f32>,
}

#[allow(dead_code)]
impl Envelope {
    pub fn time() -> Self {
        Self {
            duration: std::f32::INFINITY,
            get_value: Box::new(|time| time.clone()),
        }
    }

    pub fn hold(duration: f32, value: f32) -> Self {
        Self {
            duration,
            get_value: Box::new(move |_| value),
        }
    }

    pub fn linear(duration: f32, from: f32, to: f32) -> Self {
        Self {
            duration,
            get_value: Box::new(move |pos| {
                let t = pos / duration;
                t * to + (1.0 - t) * from
            }),
        }
    }

    pub fn catmull(duration: f32, p0: f32, p1: f32, p2: f32, p3: f32) -> Self {
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

    pub fn index(array: Vec<f32>) -> Self {
        Self {
            duration: array.last().unwrap().clone(),
            get_value: Box::new(move |pos| {
                for (i, p) in array.iter().enumerate() {
                    if pos < p {
                        return i as f32;
                    }
                }
                return array.len() as f32;
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
            duration: envelope.duration * (count as f32),
            get_value: Box::new(move |t| {
                let dt = t % envelope.duration;
                envelope.get_value(&dt)
            }),
        }
    }

    pub fn infinite(envelope: Envelope) -> Self {
        Self {
            duration: std::f32::INFINITY,
            get_value: Box::new(move |t| {
                let dt = t % envelope.duration;
                envelope.get_value(&dt)
            }),
        }
    }

    pub fn clip(duration: f32, envelope: Envelope) -> Self {
        Self {
            duration,
            get_value: Box::new(move |t| envelope.get_value(&t.min(duration))),
        }
    }

    pub fn get_value(&self, t: &f32) -> f32 {
        (self.get_value)(&t.min(self.duration))
    }

    pub fn debug(&self, t: f32) {
        println!(
            "duration = {}, value at {} = {}",
            self.duration,
            t,
            self.get_value(&t)
        );
    }
}
