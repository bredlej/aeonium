extern crate anyhow;
extern crate cpal;

mod aeonium;
mod ui;
mod common;

use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;

use crate::common::{App, ChannelPair, ThreadComm};

fn main() -> anyhow::Result<()> {
    let app = Arc::new(Mutex::new(App::default()));
    let mut app_mut = app.clone();

    let thread_comm = ThreadComm::default();

    let audio_stream = aeonium::stream_setup_for(aeonium::play_note, app, &thread_comm).unwrap();
    ui::run(audio_stream, &mut app_mut, &thread_comm)
}
