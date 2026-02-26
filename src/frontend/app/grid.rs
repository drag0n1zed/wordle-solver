use leptos::prelude::*;

use crate::backend::guess::{GuessColor, Guesses, LetterGuess};

#[derive(Default, PartialEq, Clone, Copy)]
pub(super) struct Tile {
    pub color: Option<GuessColor>,
    pub char: Option<u8>,
}

#[derive(Clone, PartialEq)]
pub(super) struct Grid {
    pub rows: usize,
    pub cols: usize,
    pub tiles: Vec<Vec<Tile>>,
}

impl Grid {
    pub(super) fn new(rows: usize, cols: usize) -> Self {
        let tiles = vec![vec![Tile::default(); cols]; rows];
        Self { rows, cols, tiles }
    }
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

#[component]
pub fn Grid(grid: RwSignal<Grid>) -> impl IntoView {
    view! {
      <div
        class="grid gap-2"
        style:grid-template-columns=move || {
          format!("repeat({}, minmax(0, 1fr))", grid.read().cols)
        }
      >
        <For
          each=move || 0..grid.read().rows
          key=|row| *row
          children=move |row| {
            view! {
              <For
                each=move || 0..grid.read().cols
                key=|col| *col
                children=move |col| {
                  view! { <Tile grid=grid row=row col=col /> }
                }
              />
            }
          }
        />
      </div>
    }
}

#[component]
fn Tile(grid: RwSignal<Grid>, row: usize, col: usize) -> impl IntoView {
    let t = Memo::new(move |_| grid.read().tiles[row][col]);

    let cycle_status = move |_| {
        grid.update(|g| {
            let t = &mut g.tiles[row][col];
            if t.char.is_some() {
                t.color = match t.color {
                    None => Some(GuessColor::Gray),
                    Some(GuessColor::Gray) => Some(GuessColor::Yellow),
                    Some(GuessColor::Yellow) => Some(GuessColor::Green),
                    Some(GuessColor::Green) => Some(GuessColor::Gray),
                };
            }
        });
    };

    view! {
      <div
        class=move || {
          let base = "w-14 h-14 sm:w-16 sm:h-16 border-2 flex items-center justify-center \
                      text-4xl font-bold uppercase cursor-pointer select-none transition-all \
                      text-white hover:brightness-90 active:scale-95";
          let state_style = match t.get().color {
            Some(GuessColor::Green) => "bg-wordle-green",
            Some(GuessColor::Yellow) => "bg-wordle-yellow",
            Some(GuessColor::Gray) => "bg-wordle-gray",
            None => "bg-white border-wordle-gray",
          };
          format!("{base} {state_style}")
        }
        on:click=cycle_status
      >
        {move || { t.get().char.map(|byte| (byte as char).to_string()).unwrap_or_default() }}
      </div>
    }
}
