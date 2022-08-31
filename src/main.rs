extern crate anyhow;
extern crate cpal;

mod aeonium;
mod ui;
mod common;

use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;

use crate::common::App;

fn main() -> anyhow::Result<()> {

    let app = Arc::new(Mutex::new(App::default()));

    let mut app_mut = app.clone();
    let (beat_sender, beat_receiver) = channel();
    let (sample_sender, sample_receiver) = channel();
    let stream = aeonium::stream_setup_for(aeonium::play_note, app, beat_sender, sample_sender).unwrap();


    ui::run(stream, &mut app_mut, &beat_receiver, sample_receiver)
}
