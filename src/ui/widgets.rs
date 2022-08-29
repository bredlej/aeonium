use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::widgets::Widget;
use crate::common::{Beat, BeatEvent};

pub struct BpmWidget<'a> {
    pub bpm: &'a u128,
    pub has_beat: bool,
}

impl<'a> Beat for BpmWidget<'a> {
    fn on_beat(&self, event : BeatEvent) {
        println!("Beat!");
    }
}

impl<'a> Widget for BpmWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let text: String;
        let style: Style;
        if self.has_beat {
            style = Style::default().fg(Color::LightGreen).add_modifier(Modifier::BOLD);
        }
        else {
            style = Style::default().fg(Color::White);
        }
        text = format!("BPM: {}", self.bpm);
        buf.set_string(area.left(), area.top(), text, style);
    }
}
