use std::path::Path;

use include_dir::{Dir, include_dir};
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

use crate::logic::{
    guess::{GuessColor, Guesses},
    reqs::Requirement,
};

static ASSETS: Dir = include_dir!("$CARGO_MANIFEST_DIR/assets");

fn get_str_asset<S>(path: S) -> Option<&'static str>
where
    S: AsRef<Path>,
{
    let bytes = ASSETS.get_file(path)?.contents();
    std::str::from_utf8(bytes).ok()
}

#[derive(Default, PartialEq, Clone, Copy)]
pub struct Tile {
    pub color: Option<GuessColor>,
    pub char: Option<u8>,
}

#[derive(Clone)]
pub struct Grid {
    pub rows: usize,
    pub cols: usize,
    pub tiles: Vec<Vec<Tile>>,
}

impl Grid {
    fn new(rows: usize, cols: usize) -> Self {
        let tiles = vec![vec![Tile::default(); cols]; rows];
        Self { rows, cols, tiles }
    }
}

#[component]
pub fn App() -> impl IntoView {
    let grid = RwSignal::new(Grid::new(3, 5));

    let resize_grid = move |new_rows: Option<usize>, new_cols: Option<usize>| {
        grid.update(|g| {
            new_rows.map(|r| g.rows = r);
            new_cols.map(|c| g.cols = c);
            g.tiles.resize(g.rows, vec![Tile::default(); g.cols]);
            for row in g.tiles.iter_mut() {
                row.resize(g.cols, Tile::default());
            }
        });
    };

    let wordlist = get_str_asset("data/enable1.txt").expect("wordlist load failed");

    let is_solvable = Memo::new(move |_| {
        grid.with(|g| {
            g.tiles
                .iter()
                .flatten()
                .all(|tile| tile.char.is_some() && tile.color.is_some())
        })
    });

    let all_solutions = RwSignal::new(Vec::<&str>::new());
    let display_count = RwSignal::new(20);

    let solve = move || {
        let guesses = grid.with(|g| Guesses::try_from(g).expect("tiles should be filled but aren't"));
        let reqs: Requirement = guesses.into();
        let results: Vec<&str> = reqs.filter_wordlist(wordlist).collect();
        all_solutions.set(results);
        display_count.set(20);
    };

    enum MoveDir {
        Left,
        Right,
        Down,
        Up,
    }

    let focus_by_grid_coords = move |row: usize, col: usize| {
        if let Some(elem) = document().get_element_by_id(format!("tile-{row}-{col}").as_str()) {
            if let Ok(input) = elem.dyn_into::<HtmlInputElement>() {
                let _ = input.focus();
            }
        }
    };

    let focus_after_moving = move |dir: MoveDir, row: usize, col: usize| {
        let (total_rows, total_cols) = {
            let g = grid.read();
            (g.rows, g.cols)
        };
        match dir {
            MoveDir::Left => {
                if col > 0 {
                    focus_by_grid_coords(row, col - 1);
                } else if row > 0 {
                    focus_by_grid_coords(row - 1, total_cols - 1);
                }
            }
            MoveDir::Right => {
                if col + 1 < total_cols {
                    focus_by_grid_coords(row, col + 1);
                } else if row + 1 < total_rows {
                    focus_by_grid_coords(row + 1, 0);
                }
            }
            MoveDir::Up => {
                if row > 0 {
                    focus_by_grid_coords(row - 1, col);
                }
            }
            MoveDir::Down => {
                if row + 1 < total_rows {
                    focus_by_grid_coords(row + 1, col);
                }
            }
        }
    };

    let handle_input = move |event: web_sys::Event, row: usize, col: usize| {
        let target: HtmlInputElement = event_target(&event);
        let val = target.value();

        let current_char = grid.with_untracked(|g| g.tiles[row][col].char.map(|b| b as char));
        let new_char = val
            .chars()
            .find(|&c| Some(c) != current_char)
            .filter(|c| c.is_ascii_alphabetic());

        if let Some(char) = new_char {
            grid.update(|g| {
                let t = &mut g.tiles[row][col];
                if t.char.is_none() {
                    t.color = Some(GuessColor::Gray);
                }
                t.char = Some(char.to_ascii_uppercase() as u8);
            });
            focus_after_moving(MoveDir::Right, row, col);
        } else {
            // Manually refresh DOM. Stops non-alphanumeric characters from being appended
            let orig_char = match current_char {
                None => String::new(),
                Some(byte) => (byte as char).to_string(),
            };
            target.set_value(&orig_char);
        }
    };

    let handle_keydown = move |event: web_sys::KeyboardEvent, row: usize, col: usize| {
        grid.update(|g| {
            let t = &mut g.tiles[row][col];
            match event.key().as_str() {
                " " => {
                    t.char = None;
                }
                "Backspace" => {
                    t.char = None;
                    focus_after_moving(MoveDir::Left, row, col);
                }
                "ArrowLeft" => {
                    focus_after_moving(MoveDir::Left, row, col);
                }
                "ArrowRight" => {
                    focus_after_moving(MoveDir::Right, row, col);
                }
                "ArrowUp" => {
                    focus_after_moving(MoveDir::Up, row, col);
                }
                "ArrowDown" => {
                    focus_after_moving(MoveDir::Down, row, col);
                }
                _ => {}
            }
        });
    };

    let cycle_status = move |row: usize, col: usize| {
        grid.update(|g| {
            let t = &mut g.tiles[row][col];

            t.color = match t.color {
                None => Some(GuessColor::Gray),
                Some(GuessColor::Gray) => Some(GuessColor::Yellow),
                Some(GuessColor::Green) => Some(GuessColor::Gray),
                Some(GuessColor::Yellow) => Some(GuessColor::Green),
            };
        });
    };

    let get_tile_view = move |row: usize, col: usize| {
        let t = Memo::new(move |_| grid.read().tiles[row][col]);

        view! {
            <input
                id=format!("tile-{row}-{col}")
                type="text"
                prop:value=move || match t.get().char {
                    None => String::new(),
                    Some(byte) => (byte as char).to_string(),
                }
                class=move || {
                    let base = "w-12 h-12 sm:w-16 sm:h-16 \
                        border-2 border-black text-center text-2xl sm:text-3xl \
                        font-bold uppercase outline-none cursor-pointer \
                        caret-transparent transition-colors focus:ring-4 \
                        focus:ring-black/20 selection:bg-transparent";
                    let color = match t.get().color {
                        None => "bg-white text-black",
                        Some(GuessColor::Gray) => "bg-wordle-gray text-white",
                        Some(GuessColor::Yellow) => "bg-wordle-yellow text-white",
                        Some(GuessColor::Green) => "bg-wordle-green text-white",
                    };
                    format!("{base} {color}")
                }
                on:focus=move |e| {
                    let _ = event_target::<HtmlInputElement>(&e).select();
                }
                on:input=move |e| handle_input(e, row, col)
                on:keydown=move |e| handle_keydown(e, row, col)
                on:click=move |_| cycle_status(row, col)
                autocomplete="off"
            />
        }
    };

    view! {
        <main class="flex flex-col items-center justify-center h-screen gap-6">
            <div class="flex items-center gap-6">
                // row count input
                <label class="flex items-center gap-2" for="guess-count">
                    "Guess count: "
                    <input
                        type="number"
                        id="guess-count"
                        prop:value=move || grid.read().rows
                        min="1"
                        max="100"
                        on:change=move |e| {
                            if let Ok(val) = event_target_value(&e).parse::<usize>() {
                                if (1..=100).contains(&val) {
                                    resize_grid(Some(val), None);
                                }
                            }
                        }
                    />
                </label>

                // word length input
                <label class="flex items-center gap-2" for="word-length">
                    "Word length: "
                    <input
                        type="number"
                        id="word-length"
                        prop:value=move || grid.read().cols
                        min="2"
                        // ethylenediaminetetraacetates
                        max="28"
                        on:change=move |e| {
                            if let Ok(val) = event_target_value(&e).parse::<usize>() {
                                if (2..=28).contains(&val) {
                                    resize_grid(None, Some(val));
                                }
                            }
                        }
                    />
                </label>
            </div>

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
                                children=move |col| get_tile_view(row, col)
                            />
                        }
                    }
                />

            </div>

            <div>
                <button
                    class="border-2 border-black p-2 px-6 \
                    font-bold uppercase bg-white hover:bg-black hover:text-white \
                    transition-colors h-[46px] ml-auto sm:ml-0 \
                    disabled:opacity-50 disabled:cursor-not-allowed disabled:pointer-events-none"
                    on:click=move |_| solve()
                    disabled=move || !is_solvable.get()
                >
                    Solve
                </button>
            </div>
            <div class="flex flex-wrap gap-2 max-w-2xl justify-center mt-4">
                {move || {
                    all_solutions
                        .get()
                        .iter()
                        .take(display_count.get())
                        .map(|word| {
                            view! {
                                <div class="bg-gray-200 px-3 py-1 rounded text-black font-semibold tracking-wider">
                                    {word.to_string()}
                                </div>
                            }
                        })
                        .collect_view()
                }}
            </div>
        </main>
    }
}
