#[derive(Clone)]
pub enum Note {
    C4,
    CSharp4,
    D4,
    DSharp4,
    E4,
    F4,
    FSharp4,
    G4,
    GSharp4,
    A4,
    ASharp4,
    B4,
    C5,
}

impl Note {
    pub(crate) fn freq(&self) -> f32 {
        match *self {
            Note::C4 => 261.,
            Note::CSharp4 => 277.,
            Note::D4 => 294.,
            Note::DSharp4 => 311.,
            Note::E4 => 330.,
            Note::F4 => 349.,
            Note::FSharp4 => 370.,
            Note::G4 => 392.,
            Note::GSharp4 => 415.,
            Note::A4 => 440.,
            Note::ASharp4 => 466.,
            Note::B4 => 493.,
            Note::C5 => 523.,
        }
    }
}