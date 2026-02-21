use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize, Deserialize)]
pub enum GuessColor {
    #[default]
    Gray,
    Yellow,
    Green,
}

#[derive(Serialize, Deserialize)]
pub struct LetterGuess {
    pub color: GuessColor,
    pub char: u8,
}

#[derive(Serialize, Deserialize)]
pub struct Guesses {
    pub word_len: usize,
    pub val: Vec<LetterGuess>,
}
