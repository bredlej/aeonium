pub mod ui;
pub use ui::*;

enum InputMode {
    Normal,
    Editing,
}

/// App holds the state of the application
pub struct App {
    pub bpm: u128
}

impl Default for App {
    fn default() -> App {
        App {
            bpm: 320
        }
    }
}