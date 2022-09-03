pub trait Waveform {
    fn tone(&self, frequency: f32) -> f32;
    fn tick(&mut self);
}

pub struct SinWave {
    pub sample_rate: f32,
    pub sample_clock: f32,
    pub nchannels: usize,
}

impl Waveform for SinWave {
    fn tone(&self, freq: f32) -> f32 {
        (self.sample_clock * freq * 2.0 * std::f32::consts::PI / self.sample_rate).sin()
    }
    fn tick(&mut self) {
        self.sample_clock = (self.sample_clock + 1.0) % self.sample_rate;
    }
}