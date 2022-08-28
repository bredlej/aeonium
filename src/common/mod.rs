use std::ops::Not;
use std::sync::{Arc, Mutex};

pub struct Notifier<E> {
    subscribers: Vec<Box<dyn Fn(&E) + Send>>,
}

impl<E> Notifier<E> {
    pub fn new() -> Notifier<E> {
        Notifier {
            subscribers: Vec::new(),
        }
    }

    pub fn register<F>(&mut self, callback: F)
        where
            F: 'static + Fn(&E) + Send,
    {
        self.subscribers.push(Box::new(callback));
    }

    pub fn notify(&self, event: E) {
        for callback in &self.subscribers {
            callback(&event);
        }
    }
}

pub struct BeatEvent {}

pub trait Beat {
    fn on_beat(&self, beat_event: BeatEvent);
}

pub struct App {
    pub bpm: u128,
    pub beat_notifier : Notifier<BeatEvent>
}

impl Default for App {
    fn default() -> App {
        App {
            bpm: 60,
            beat_notifier: Notifier::new(),
        }
    }
}