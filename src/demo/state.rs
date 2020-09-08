use crate::create_state;
use crate::scripting::*;

pub struct State {
    pub time: f64,
    pub cam_x: f64,
    pub cam_y: f64,
    pub cam_z: f64,
}

impl State {
    pub fn new() -> StateFn<Self> {
        create_state!(Self {
            time => Envelope::time(),
            cam_x => Envelope::infinite(Envelope::linear(12.0, 4.0, 2.0)),
            cam_y => Envelope::infinite(Envelope::linear(8.0, 6.0, 2.0)),
            cam_z => Envelope::infinite(Envelope::linear(15.0, 7.0, 2.0))
        })
    }
}
