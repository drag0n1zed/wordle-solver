use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum GuessType {
    Green,
    Yellow,
    Gray,
}

#[derive(Serialize, Deserialize)]
pub struct CharGuess {
    pub color: GuessType,
    pub char: u8,
}

#[derive(Serialize, Deserialize)]
pub struct Guesses {
    pub word_len: usize,
    pub val: Vec<CharGuess>,
}
