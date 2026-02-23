use leptos::prelude::{Get, GetUntracked, ReadUntracked, RwSignal};
use serde::{Deserialize, Serialize};

use crate::frontend::app::{Grid, Tile};

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
    pub fn from_grid(grid: &Grid) -> Option<Self> {
        let word_len = grid.cols;
        let val: Vec<LetterGuess> = grid
            .tiles
            .iter()
            .flatten()
            .map(|signal| {
                let tile = signal.read_untracked();
                Some(LetterGuess {
                    color: tile.color?,
                    char: tile.char?,
                })
            })
            .collect::<Option<Vec<LetterGuess>>>()?;
        Some(Self { word_len, val })
    }
}
