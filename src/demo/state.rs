use crate::create_state;
use crate::scripting::*;

#[derive(Debug)]
pub struct State {
    pub time: f32,
    pub part: f32,
    pub strobe: f32,
    pub speed: f32,
    pub rotation: f32,
    pub mesh_pumping_factor: f32,
    pub fade: f32,
}

const MARKERS: [f32; 17] = [
    0.524, 4.946, 9.080, 13.078, 16.803, 20.683, 24.375, 28.030, 31.738, 35.476, 39.166, 42.865,
    46.594, 50.115, 53.592, 57.078, 60.129,
];

impl State {
    pub fn new() -> StateFn<Self> {
        let strobe = State::strobe();
        create_state!(Self {
            time => Envelope::time(),
            part => Envelope::index(Vec::<f32>::from(MARKERS)),
            strobe => strobe,
            speed => Envelope::concat(vec![
                Envelope::hold(31.738, 1.0),
                Envelope::linear(15.0, 1.0, 2.0),
                Envelope::linear(14.0, 2.0, 2.5),
                Envelope::linear(0.5, 2.5, 1.0),
            ]),
            rotation => Envelope::concat(vec![
                Envelope::linear(31.0, 0.0, 100.0),
                Envelope::linear(10.0, 100.0, 140.0),
                Envelope::linear(10.0, 140.0, 190.0),
                Envelope::linear(10.0, 190.0, 250.0),
            ]),
            mesh_pumping_factor => Envelope::concat(vec![
                Envelope::hold(46.594, 0.0),
                Envelope::linear(15.0, 0.1, 0.5),
            ]),
            fade => Envelope::concat(vec![
                Envelope::linear(0.524, 0.0, 1.0),
                Envelope::hold(60.0, 1.0),
                Envelope::linear(2.0, 1.0, 0.0)
            ])
        })
    }

    fn strobe() -> Envelope {
        Envelope::concat(
            MARKERS
                .iter()
                .scan(0.0, |state, &time| {
                    let prev_time = state.clone();
                    *state = time;
                    Some(time - prev_time)
                })
                .flat_map(|duration| {
                    let dur = duration / 8.0;
                    vec![
                        Envelope::linear(dur, 1.0, 0.0),
                        Envelope::linear(dur, 0.5, 0.0),
                        Envelope::linear(dur, 0.8, 0.0),
                        Envelope::linear(dur, 0.8, 0.0),
                        Envelope::linear(dur, 1.0, 0.0),
                        Envelope::linear(dur, 0.5, 0.0),
                        Envelope::linear(dur, 0.8, 0.0),
                        Envelope::linear(dur, 0.8, 0.0),
                    ]
                })
                .collect(),
        )
    }
}
