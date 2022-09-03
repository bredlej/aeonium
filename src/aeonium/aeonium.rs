use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Sender};
use std::time::Instant;
use cpal::traits::{DeviceTrait, HostTrait};
use crate::aeonium::music::Note;
use crate::aeonium::waveform::{SinWave, Waveform};
use crate::App;
use crate::common::BeatEvent;

pub fn play_note(waveform: &mut dyn Waveform, frequency: f32, velocity: f32) -> f32 {
    waveform.tick();
    waveform.tone(frequency) * velocity
}

pub fn stream_setup_for<F>(on_sample: F, app: Arc<Mutex<App>>, beat_sender: Sender<BeatEvent>, sample_sender: Sender<Vec<f32>>) -> Result<cpal::Stream, anyhow::Error>
    where
        F: FnMut(&mut dyn Waveform, f32, f32) -> f32 + std::marker::Send + 'static + Copy,
{
    let (_host, device, config) = host_device_setup()?;

    match config.sample_format() {
        cpal::SampleFormat::F32 => stream_make::<f32, _>(&device, &config.into(), on_sample, app, beat_sender, sample_sender),
        cpal::SampleFormat::I16 => stream_make::<i16, _>(&device, &config.into(), on_sample, app, beat_sender, sample_sender),
        cpal::SampleFormat::U16 => stream_make::<u16, _>(&device, &config.into(), on_sample, app, beat_sender, sample_sender),
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
    sound_function: F,
    app: Arc<Mutex<App>>,
    beat_sender: Sender<BeatEvent>,
    sample_sender: Sender<Vec<f32>>,
) -> Result<cpal::Stream, anyhow::Error>
    where
        T: cpal::Sample,
        F: FnMut(&mut dyn Waveform, f32, f32) -> f32 + std::marker::Send + 'static + Copy,
{
    let sample_rate = config.sample_rate.0 as f32;
    let sample_clock = 0f32;
    let nchannels = config.channels as usize;
    let mut request = SinWave {
        sample_rate,
        sample_clock,
        nchannels,
    };
    let err_fn = |err| eprintln!("Error building output sound stream: {}", err);

    let track: Vec<Note> = vec![Note::C4, Note::D4, Note::E4, Note::F4, Note::G4, Note::A4, Note::B4, Note::C5];
    let track_len = track.len();

    let mut time = Instant::now();
    let mut i = 0;

    let stream = device.build_output_stream(
        config,
        move |output: &mut [T], oci: &cpal::OutputCallbackInfo| {
            if time.elapsed().as_millis() >= bpm_to_miliseconds(&app.lock().unwrap().bpm) {
                beat_sender.send(BeatEvent{note: track[i].clone()}).unwrap();
                time = Instant::now();
                i += 1;
                if i == track_len { i = 0 };
            }
            update_sample_buffer(output, &mut request, sound_function, track[i].freq(), &sample_sender)
        },
        err_fn,
    )?;

    Ok(stream)
}

fn update_sample_buffer<T, F>(output: &mut [T], request: &mut SinWave, mut sound_function: F, note: f32, sample_sender: &Sender<Vec<f32>>,)
    where
        T: cpal::Sample,
        F: FnMut(&mut dyn Waveform, f32, f32) -> f32 + std::marker::Send + 'static,
{
    let mut samples = vec![];
    samples.clear();
    for frame in output.chunks_mut(request.nchannels) {
        let value: T = cpal::Sample::from::<f32>(&sound_function(request, note, 1.0));
        for sample in frame.iter_mut() {
            *sample = value;
            samples.push(value.to_f32());
        }
    }
    sample_sender.send(samples);
}