use std::sync::mpsc::{channel, Sender};
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::Style;
use tui::widgets::Widget;
use crate::common::{Beat, BeatEvent};

pub struct BpmWidget<'a> {
    pub bpm: &'a u128,
}

impl<'a> Beat for BpmWidget<'a> {
    fn on_beat(&self, event : BeatEvent) {
        println!("Beat!");
    }
}

impl<'a> Widget for BpmWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (tx, rx): (Sender<BeatEvent>, std::sync::mpsc::Receiver<BeatEvent>) = channel();
        let beat_event = rx.try_recv();
        match beat_event {
            Ok(_) => {println!("Beat");}
            Err(_) => {}
        }
        let text = format!("BPM: {}", self.bpm);
        buf.set_string(area.left(), area.top(), text, Style::default());
    }
}

impl<'a> BpmWidget<'a> {
    fn text(mut self, bpm: &'a u128) -> BpmWidget<'a> {
        self.bpm = bpm;
        self
    }
}