extern crate anyhow;
extern crate cpal;

mod aeonium;
mod ui;

use std::sync::{Arc, Mutex};

use cpal::traits::{StreamTrait};
use crate::ui::App;


fn main() -> anyhow::Result<()> {

    let app = Arc::new(Mutex::new(App::default()));
    let mut app_mut = app.clone();
    let stream = aeonium::stream_setup_for(aeonium::play_note, app).unwrap();

    ui::run(stream, &mut app_mut)
}
