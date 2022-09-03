use std::sync::mpsc::{channel, Receiver, Sender};
use crate::aeonium::music::Note;


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

pub struct ChannelPair<T> {
    pub tx: Sender<T>,
    pub rx: Receiver<T>,
}

impl<T> ChannelPair<T> {
    pub(crate) fn new() -> Self {
        let (tx, rx) = channel();
        Self { tx, rx }
    }
}

pub struct ThreadComm {
    pub beats: ChannelPair<BeatEvent>,
    pub samples: ChannelPair<Vec<f32>>,
}

impl Default for ThreadComm {
    fn default() -> Self {
        ThreadComm {
            beats: ChannelPair::new(),
            samples: ChannelPair::new(),
        }
    }
}
