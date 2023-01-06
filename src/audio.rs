use cpal::{Host, Stream};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

pub struct Audio {
    stream: Stream,
}

impl Audio {
    pub fn new() -> Self {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("failed to find a default output device");
        let config = device.default_output_config().unwrap();

        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => Self::run::<f32>(&device, &config.into()),
            cpal::SampleFormat::I16 => Self::run::<i16>(&device, &config.into()),
            cpal::SampleFormat::U16 => Self::run::<u16>(&device, &config.into()),
        };
        Audio { stream }
    }

    pub fn play(&self) {
        self.stream.play().unwrap();
    }

    pub fn pause(&self) {
        self.stream.pause().unwrap();
    }

    fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig) -> Stream
        where
            T: cpal::Sample,
    {
        let sample_rate = config.sample_rate.0 as f32;
        let channels = config.channels as usize;

        // Produce a sinusoid of maximum amplitude.
        let mut sample_clock = 0f32;
        let mut next_value = move || {
            sample_clock = (sample_clock + 1.0) % sample_rate;
            (sample_clock * 440.0 * 2.0 * 3.141592 / sample_rate).sin()
        };

        let err_fn = |err| println!("an error occurred on stream: {}", err).into();

        let stream = device
            .build_output_stream(
                config,
                move |data: &mut [T], _| Self::write_data(data, channels, &mut next_value),
                err_fn,
            )
            .unwrap();
        stream.pause().unwrap();
        stream
    }

    fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
        where
            T: cpal::Sample,
    {
        for frame in output.chunks_mut(channels) {
            let value: T = cpal::Sample::from::<f32>(&next_sample());
            for sample in frame.iter_mut() {
                *sample = value;
            }
        }
    }
}