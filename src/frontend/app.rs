use std::path::Path;

use include_dir::{Dir, include_dir};
use leptos::prelude::*;

use crate::logic::guess::GuessColor;

static ASSETS: Dir = include_dir!("$CARGO_MANIFEST_DIR/assets");

fn get_str_asset<S>(path: S) -> Option<&'static str>
where
    S: AsRef<Path>,
{
    let bytes = ASSETS.get_file(path)?.contents();
    std::str::from_utf8(bytes).ok()
}

#[derive(Default, Clone)]
pub struct Tile {
    pub color: GuessColor,
    pub char: Option<u8>,
}

fn cycle_status(grid: RwSignal<Vec<Vec<Tile>>>, row: usize, col: usize) {
    grid.update(|g| {
        g[row][col].color = match g[row][col].color {
            GuessColor::Gray => GuessColor::Green,
            GuessColor::Green => GuessColor::Yellow,
            GuessColor::Yellow => GuessColor::Gray,
        };
    });
}

fn handle_input(
    grid: RwSignal<Vec<Vec<Tile>>>,
    event: web_sys::Event,
    row: usize,
    col: usize,
    word_len: usize,
    total_rows: usize,
) {
    let val = event_target_value(&event);
    let char = val.chars().last().and_then(|char| {
        if char.is_ascii_alphabetic() {
            Some(char.to_ascii_uppercase() as u8)
        } else {
            None
        }
    });
    grid.update(|g| g[row][col].char = char);

    // focus next tile if valid input
    if char.is_some() && (col + 1 < word_len || row + 1 < total_rows) {
        if col + 1 < word_len {
            // TODO: Focus on (row, col + 1)
        } else if row + 1 < total_rows {
            // TODO: Focus on (row + 1, 0)
        }
        // else last input box, do nothing
    }
}

#[component]
pub fn App() -> impl IntoView {
    let wordlist = get_str_asset("data/enable1.txt").expect("wordlist load failed");

    let rows = RwSignal::new(3);
    let cols = RwSignal::new(5);

    let grid = RwSignal::new(vec![vec![Tile::default(); cols.get_untracked()]; rows.get_untracked()]);

    // reactive grid resize
    Effect::new(move || {
        let new_rows = rows.get();
        let new_cols = cols.get();
        grid.update(|g| {
            g.resize(new_rows, vec![Tile::default(); new_cols]);
            for row in g.iter_mut() {
                row.resize(new_cols, Tile::default());
            }
        })
    });

    view! {}
}
