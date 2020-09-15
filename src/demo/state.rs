use crate::create_state;
use crate::scripting::*;

pub struct State {
    pub time: f32,
    pub part: f32,
}

const MARKERS: [f32; 17] = [
    0.524, 4.946, 9.080, 13.078, 16.803, 20.683, 24.375, 28.030, 31.738, 35.476, 39.166, 42.865,
    46.594, 50.115, 53.592, 57.078, 60.129,
];

impl State {
    pub fn new() -> StateFn<Self> {
        create_state!(Self {
            time => Envelope::time(),
            part => Envelope::index(Vec::<f32>::from(MARKERS))
        })
    }
}
