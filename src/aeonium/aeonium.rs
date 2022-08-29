use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Sender};
use std::time::Instant;
use cpal::traits::{DeviceTrait, HostTrait};
use crate::aeonium::{music, Note};
use crate::App;
use crate::common::BeatEvent;

pub fn play_note(o: &mut SampleRequestOptions, note: f32) -> f32 {
    o.tick();
    o.tone(note) * 1.
}

pub struct SampleRequestOptions {
    pub sample_rate: f32,
    pub sample_clock: f32,
    pub nchannels: usize,
}

impl SampleRequestOptions {
    fn tone(&self, freq: f32) -> f32 {
        (self.sample_clock * freq * 2.0 * std::f32::consts::PI / self.sample_rate).sin()
    }
    fn tick(&mut self) {
        self.sample_clock = (self.sample_clock + 1.0) % self.sample_rate;
    }
}

pub fn stream_setup_for<F>(on_sample: F, app: Arc<Mutex<App>>, beat_sender: Sender<BeatEvent>) -> Result<cpal::Stream, anyhow::Error>
    where
        F: FnMut(&mut SampleRequestOptions, f32) -> f32 + std::marker::Send + 'static + Copy,
{
    let (_host, device, config) = host_device_setup()?;

    match config.sample_format() {
        cpal::SampleFormat::F32 => stream_make::<f32, _>(&device, &config.into(), on_sample, app, beat_sender),
        cpal::SampleFormat::I16 => stream_make::<i16, _>(&device, &config.into(), on_sample, app, beat_sender),
        cpal::SampleFormat::U16 => stream_make::<u16, _>(&device, &config.into(), on_sample, app, beat_sender),
    }
}

pub fn host_device_setup() -> Result<(cpal::Host, cpal::Device, cpal::SupportedStreamConfig), anyhow::Error> {
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .ok_or_else(|| anyhow::Error::msg("Default output device is not available"))?;
    println!("Output device : {}", device.name()?);

    let config = device.default_output_config()?;
    println!("Default output config : {:?}", config);

    Ok((host, device, config))
}

fn bpm_to_miliseconds(bpm: &u128) -> u128 {
    60000 / bpm
}

pub fn stream_make<T, F>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    on_sample: F,
    app: Arc<Mutex<App>>,
    beat_sender: Sender<BeatEvent>,
) -> Result<cpal::Stream, anyhow::Error>
    where
        T: cpal::Sample,
        F: FnMut(&mut SampleRequestOptions, f32) -> f32 + std::marker::Send + 'static + Copy,
{
    let sample_rate = config.sample_rate.0 as f32;
    let sample_clock = 0f32;
    let nchannels = config.channels as usize;
    let mut request = SampleRequestOptions {
        sample_rate,
        sample_clock,
        nchannels,
    };
    let err_fn = |err| eprintln!("Error building output sound stream: {}", err);

    let track: Vec<Note> = vec![music::Note::C4, music::Note::D4, music::Note::E4, music::Note::F4, music::Note::G4, music::Note::A4, music::Note::B4, music::Note::C5];
    let track_len = track.len();

    let mut time = Instant::now();
    let mut i = 0;

    let stream = device.build_output_stream(
        config,
        move |output: &mut [T], _: &cpal::OutputCallbackInfo| {
            if time.elapsed().as_millis() >= bpm_to_miliseconds(&app.lock().unwrap().bpm) {
                beat_sender.send(BeatEvent{note: track[i].clone()}).unwrap();
                time = Instant::now();
                i += 1;
                if i == track_len { i = 0 };
            }
            on_window(output, &mut request, on_sample, track[i].freq())
        },
        err_fn,
    )?;

    Ok(stream)
}

fn on_window<T, F>(output: &mut [T], request: &mut SampleRequestOptions, mut on_sample: F, note: f32)
    where
        T: cpal::Sample,
        F: FnMut(&mut SampleRequestOptions, f32) -> f32 + std::marker::Send + 'static,
{
    for frame in output.chunks_mut(request.nchannels) {
        let value: T = cpal::Sample::from::<f32>(&on_sample(request, note));
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}