use cpal::traits::{DeviceTrait, HostTrait};
use minimp3::{Decoder, Error, Frame};
use std::sync::{Arc, Mutex};

pub struct Music {
    buffer: Arc<Vec<i16>>,
    sample_rate: cpal::SampleRate,
    channels: cpal::ChannelCount,
    stream: Option<cpal::Stream>,
    position: Arc<Mutex<usize>>,
}

#[allow(dead_code)]
impl Music {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut decoder = Decoder::new(bytes);
        let mut buffer = Vec::new();
        let mut sample_rate = cpal::SampleRate(0);
        let mut channels: cpal::ChannelCount = 1;

        loop {
            match decoder.next_frame() {
                Ok(Frame {
                    mut data,
                    sample_rate: rate,
                    channels: ch,
                    ..
                }) => {
                    sample_rate = cpal::SampleRate(rate as u32);
                    channels = ch as cpal::ChannelCount;
                    buffer.append(&mut data);
                }
                Err(Error::Eof) => break,
                Err(e) => panic!("{:?}", e),
            }
        }

        Self {
            buffer: Arc::new(buffer),
            sample_rate,
            channels,
            stream: None,
            position: Arc::new(Mutex::new(0)),
        }
    }

    pub fn play(&mut self) {
        let host = cpal::default_host();

        let device = host
            .default_output_device()
            .expect("no output device available");

        let mut supported_configs_range = device
            .supported_output_configs()
            .expect("error while querying configs");

        let supported_config = supported_configs_range
            .find(|range| {
                range.sample_format() == cpal::SampleFormat::F32
                    && range.max_sample_rate() >= self.sample_rate
                    && range.min_sample_rate() <= self.sample_rate
                    && range.channels() == self.channels
            })
            .expect("Could not find supported audio config")
            .with_sample_rate(self.sample_rate);

        println!("Play music");
        let buffer = self.buffer.clone();
        let position = self.position.clone();

        self.stream = Some(
            device
                .build_output_stream(
                    &supported_config.into(),
                    move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                        let mut pos = position.lock().unwrap();
                        for sample in data.iter_mut() {
                            let value = if *pos < buffer.len() { buffer[*pos] } else { 0 };
                            *sample = cpal::Sample::from(&value);
                            *pos += 1;
                        }
                    },
                    move |err| panic!(err),
                )
                .expect("Building output stream failed"),
        );
    }

    pub fn set_position(&mut self, seconds: f64) {
        let mut position = self.position.lock().unwrap();
        *position = self.seconds_to_samples(seconds).max(0) as usize;
    }

    pub fn forward(&mut self, seconds: f64) {
        let number_of_samples = self.seconds_to_samples(seconds);
        let mut position = self.position.lock().unwrap();
        *position = (*position as i32 + number_of_samples).max(0) as usize;
    }

    fn seconds_to_samples(&self, seconds: f64) -> i32 {
        (self.sample_rate.0 as f64 * seconds) as i32 * self.channels as i32
    }
}
