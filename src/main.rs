extern crate anyhow;
extern crate clap;
extern crate cpal;

mod aeonium;

use cpal::traits::{StreamTrait};

fn main() -> anyhow::Result<()> {
    let stream = aeonium::stream_setup_for(aeonium::sample_next)?;
    stream.play()?;
    loop {}
}
