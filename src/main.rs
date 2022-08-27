extern crate anyhow;
extern crate cpal;

mod aeonium;
mod ui;

use cpal::traits::{StreamTrait};
use crate::ui::App;


fn main() -> anyhow::Result<()> {

    let mut app = App::default();
    let stream = aeonium::stream_setup_for(aeonium::play_note, &mut app).unwrap();

    ui::run(stream, &mut app)
}
