use serde::{Deserialize, Serialize};

use crate::frontend::app::Tile;

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
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

impl Guesses {
    // None if grid contains tile with no color or no character
    pub fn from_grid(grid: Vec<Vec<Tile>>, word_len: usize) -> Option<Self> {
        let val: Vec<LetterGuess> = grid
            .iter()
            .flatten()
            .map(|tile| {
                Some(LetterGuess {
                    color: tile.color?,
                    char: tile.char?,
                })
            })
            .collect::<Option<Vec<LetterGuess>>>()?;
        Some(Self { word_len, val })
    }
}
