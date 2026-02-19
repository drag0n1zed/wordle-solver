pub enum GuessType {
    Green,
    Yellow,
    Gray,
}

pub struct CharGuess {
    pub color: GuessType,
    pub char: u8,
}

pub struct Guesses {
    pub word_len: usize,
    pub val: Vec<CharGuess>,
}
