use crate::aeonium::Note;

pub struct BeatEvent {
    pub note: Note,
}

pub trait Beat {
    fn on_beat(&self, beat_event: BeatEvent);
}

pub struct App {
    pub bpm: u128,
}

impl Default for App {
    fn default() -> App {
        App {
            bpm: 60,
        }
    }
}