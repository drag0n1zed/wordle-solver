use serde::{Deserialize, Serialize};

use crate::frontend::app::Grid;

#[derive(Default, PartialEq, Clone, Copy, Serialize, Deserialize)]
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

impl TryFrom<&Grid> for Guesses {
    type Error = ();

    fn try_from(grid: &Grid) -> Result<Self, Self::Error> {
        let val = grid
            .tiles
            .iter()
            .flatten()
            .map(|tile| {
                Some(LetterGuess {
                    color: tile.color?,
                    char: tile.char?,
                })
            })
            .collect::<Option<Vec<_>>>()
            .ok_or(())?;

        Ok(Self {
            word_len: grid.cols,
            val,
        })
    }
}
