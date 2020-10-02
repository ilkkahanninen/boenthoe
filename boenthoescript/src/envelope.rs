use crate::vector::Vector;

type Duration = f64;

pub trait Envelope {
    fn get_duration(&self) -> Duration;
    fn get_value(&self, time: Duration) -> Vector;
}

pub struct Hold {
    pub duration: Duration,
    pub value: Vector,
}

impl Hold {
    pub fn new(duration: Duration, value: Vector) -> Self {
        Self { duration, value }
    }
}

impl Envelope for Hold {
    fn get_duration(&self) -> Duration {
        self.duration
    }

    fn get_value(&self, _time: Duration) -> Vector {
        self.value.clone()
    }
}

pub struct Linear {
    pub duration: f64,
    pub a: Vector,
    pub b: Vector,
}

impl Linear {
    pub fn new(duration: Duration, from: Vector, to: Vector) -> Self {
        Self {
            duration,
            a: from.clone(),
            b: &to - &from,
        }
    }
}

impl Envelope for Linear {
    fn get_duration(&self) -> Duration {
        self.duration
    }

    fn get_value(&self, time: Duration) -> Vector {
        let t = (time / self.duration).min(1.0);
        &self.a + &self.b.scalar(t)
    }
}

pub struct Concat {
    pub duration: Duration,
    pub cons: Vec<Box<dyn Envelope>>,
}

impl Concat {
    pub fn new(cons: Vec<Box<dyn Envelope>>) -> Self {
        let duration = cons.iter().map(|a| a.get_duration()).sum();
        Self { duration, cons }
    }
}

impl Envelope for Concat {
    fn get_duration(&self) -> Duration {
        self.duration
    }

    fn get_value(&self, time: Duration) -> Vector {
        let mut pos = 0.0;
        let mut last_duration = 0.0;
        let hit = self.cons.iter().find(|a| {
            last_duration = a.get_duration();
            pos += last_duration;
            time < pos
        });
        let shifted_time = time - pos + last_duration;
        match hit {
            Some(e) => e.get_value(shifted_time),
            None => match self.cons.last() {
                Some(e) => e.get_value(shifted_time),
                None => 0.0.into(), // empty list
            },
        }
    }
}

pub struct Repeat {
    pub duration: Duration,
    pub repeats: u32,
    pub envelope: Box<dyn Envelope>,
}

impl Repeat {
    pub fn new(repeats: u32, envelope: Box<dyn Envelope>) -> Self {
        let duration = envelope.get_duration() * (repeats as Duration);
        Self {
            repeats,
            envelope,
            duration,
        }
    }
}

impl Envelope for Repeat {
    fn get_duration(&self) -> Duration {
        self.duration
    }

    fn get_value(&self, time: Duration) -> Vector {
        self.envelope.get_value(if time > self.duration {
            time
        } else {
            time % self.envelope.get_duration()
        })
    }
}

pub struct Loop {
    pub envelope: Box<dyn Envelope>,
}

impl Loop {
    pub fn new(envelope: Box<dyn Envelope>) -> Self {
        Self { envelope }
    }
}

impl Envelope for Loop {
    fn get_duration(&self) -> Duration {
        std::f64::INFINITY
    }

    fn get_value(&self, time: Duration) -> Vector {
        self.envelope.get_value(time % self.envelope.get_duration())
    }
}

#[test]
fn hold() {
    let x = Hold::new(1.0, 10.0.into());
    assert_eq!(x.get_duration(), 1.0);
    assert_eq!(x.get_value(0.0).to_f(), 10.0);
    assert_eq!(x.get_value(10.0).to_f(), 10.0);

    let x = Hold::new(1.0, vec![10.0, 5.0].into());
    assert_eq!(x.get_value(0.0), Vector::from(vec![10.0, 5.0]));
}

#[test]
fn linear() {
    let x = Linear::new(1.0, 10.0.into(), 20.0.into());
    assert_eq!(x.get_duration(), 1.0);
    assert_eq!(x.get_value(0.0).to_f(), 10.0);
    assert_eq!(x.get_value(0.5).to_f(), 15.0);
    assert_eq!(x.get_value(1.5).to_f(), 20.0);
    assert_eq!(x.get_value(1.5).to_f(), 20.0);

    let x = Linear::new(1.0, vec![10.0, 10.0].into(), vec![20.0, 0.0].into());
    assert_eq!(x.get_value(0.0).to_f2(), (10.0, 10.0));
    assert_eq!(x.get_value(0.5).to_f2(), (15.0, 5.0));
    assert_eq!(x.get_value(1.0).to_f2(), (20.0, 0.0));
}

#[test]
fn concat() {
    let x = Concat::new(vec![
        Box::new(Hold::new(1.0, 0.0.into())),
        Box::new(Linear::new(1.0, 1.0.into(), 0.0.into())),
    ]);

    assert_eq!(x.get_duration(), 2.0);
    assert_eq!(x.get_value(0.0).to_f(), 0.0);
    assert_eq!(x.get_value(0.5).to_f(), 0.0);
    assert_eq!(x.get_value(1.0).to_f(), 1.0);
    assert_eq!(x.get_value(1.5).to_f(), 0.5);
    assert_eq!(x.get_value(2.0).to_f(), 0.0);
    assert_eq!(x.get_value(2.5).to_f(), 0.0);

    let x = Concat::new(vec![
        Box::new(Hold::new(2.0, vec![0.0, 1.0].into())),
        Box::new(Linear::new(
            2.0,
            vec![0.0, 1.0].into(),
            vec![1.0, 0.0].into(),
        )),
    ]);

    assert_eq!(x.get_duration(), 4.0);
    assert_eq!(x.get_value(0.0).to_f2(), (0.0, 1.0));
    assert_eq!(x.get_value(3.0).to_f2(), (0.5, 0.5));
    assert_eq!(x.get_value(4.0).to_f2(), (1.0, 0.0));
}
