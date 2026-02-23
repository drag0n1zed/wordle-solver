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

#[derive(Default, Clone)]
pub struct Tile {
    pub color: Option<GuessColor>,
    pub char: Option<u8>,
}

enum MoveDir {
    Left,
    Right,
    Down,
    Up,
}

fn update_grid_size(grid: RwSignal<Vec<Vec<RwSignal<Tile>>>>, new_rows: usize, new_cols: usize) {
    grid.update(|g| {
        g.resize_with(new_rows, || {
            (0..new_cols).map(|_| RwSignal::new(Tile::default())).collect()
        });
        for row in g.iter_mut() {
            row.resize_with(new_cols, || RwSignal::new(Tile::default()));
        }
    });
}

fn is_grid_solvable(grid: RwSignal<Vec<Vec<RwSignal<Tile>>>>) -> bool {
    grid.get()
        .iter()
        .flatten()
        .all(|tile| tile.get().char.is_some() && tile.get().color.is_some())
}

fn solve_action(
    grid: RwSignal<Vec<Vec<RwSignal<Tile>>>>,
    cols: RwSignal<usize>,
    wordlist: &'static str,
    all_solutions: RwSignal<Vec<&'static str>>,
    display_count: RwSignal<usize>,
) {
    let guesses = Guesses::from_grid(grid.get(), cols.get()).expect("tiles should be filled but aren't");
    let reqs: Requirement = guesses.into();
    let results: Vec<&str> = reqs.filter_wordlist(wordlist).collect();
    all_solutions.set(results);
    display_count.set(20);
}

fn focus_by_grid_coords(row: usize, col: usize) {
    if let Some(elem) = document().get_element_by_id(format!("tile-{row}-{col}").as_str()) {
        if let Ok(input) = elem.dyn_into::<HtmlInputElement>() {
            let _ = input.focus();
        }
    }
}

fn focus_after_moving(dir: MoveDir, row: usize, col: usize, rows: RwSignal<usize>, cols: RwSignal<usize>) {
    let (total_rows, total_cols) = (rows.get_untracked(), cols.get_untracked());
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
}

fn handle_input(
    event: web_sys::Event,
    row: usize,
    col: usize,
    grid: RwSignal<Vec<Vec<RwSignal<Tile>>>>,
    rows: RwSignal<usize>,
    cols: RwSignal<usize>,
) {
    let target: HtmlInputElement = event_target(&event);
    let val = target.value();

    let current_char = grid.get_untracked()[row][col].get_untracked().char.map(|b| b as char);
    let new_char = val
        .chars()
        .find(|&c| Some(c) != current_char)
        .filter(|c| c.is_ascii_alphabetic());

    if let Some(char) = new_char {
        grid.update(|g| {
            let signal = g[row][col];
            if signal.get_untracked().char.is_none() {
                signal.update(|t| t.color = Some(GuessColor::Gray));
            }
            signal.update(|t| t.char = Some(char.to_ascii_uppercase() as u8));
        });
        focus_after_moving(MoveDir::Right, row, col, rows, cols);
    } else {
        // Manually refresh DOM. Stops non-alphanumeric characters from being appended
        let orig_char = match grid.get_untracked()[row][col].get_untracked().char {
            None => String::new(),
            Some(byte) => (byte as char).to_string(),
        };
        target.set_value(&orig_char);
    }
}

fn handle_keydown(
    event: web_sys::KeyboardEvent,
    row: usize,
    col: usize,
    grid: RwSignal<Vec<Vec<RwSignal<Tile>>>>,
    rows: RwSignal<usize>,
    cols: RwSignal<usize>,
) {
    match event.key().as_str() {
        " " => {
            grid.update(|g| g[row][col].update(|t| t.char = None));
        }
        "Backspace" => {
            grid.update(|g| g[row][col].update(|t| t.char = None));
            focus_after_moving(MoveDir::Left, row, col, rows, cols);
        }
        "ArrowLeft" => {
            focus_after_moving(MoveDir::Left, row, col, rows, cols);
        }
        "ArrowRight" => {
            focus_after_moving(MoveDir::Right, row, col, rows, cols);
        }
        "ArrowUp" => {
            focus_after_moving(MoveDir::Up, row, col, rows, cols);
        }
        "ArrowDown" => {
            focus_after_moving(MoveDir::Down, row, col, rows, cols);
        }
        _ => {}
    }
}

fn cycle_status(row: usize, col: usize, grid: RwSignal<Vec<Vec<RwSignal<Tile>>>>) {
    let t_sig = grid.get_untracked()[row][col];

    t_sig.update(|t| {
        t.color = match t.color {
            None => Some(GuessColor::Gray),
            Some(GuessColor::Gray) => Some(GuessColor::Yellow),
            Some(GuessColor::Green) => Some(GuessColor::Gray),
            Some(GuessColor::Yellow) => Some(GuessColor::Green),
        }
    });
}

fn get_tile_view(
    row: usize,
    col: usize,
    grid: RwSignal<Vec<Vec<RwSignal<Tile>>>>,
    rows: RwSignal<usize>,
    cols: RwSignal<usize>,
) -> impl IntoView {
    let t_sig = grid.get_untracked()[row][col];
    view! {
        <input
            id=format!("tile-{row}-{col}")
            type="text"
            prop:value=move || match t_sig.get().char {
                None => String::new(),
                Some(byte) => (byte as char).to_string(),
            }
            class=move || {
                let base = "w-12 h-12 sm:w-16 sm:h-16 \
                    border-2 border-black text-center text-2xl sm:text-3xl \
                    font-bold uppercase outline-none cursor-pointer \
                    caret-transparent transition-colors focus:ring-4 \
                    focus:ring-black/20 selection:bg-transparent";
                let color = match t_sig.get().color {
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
            on:input=move |e| handle_input(e, row, col, grid, rows, cols)
            on:keydown=move |e| handle_keydown(e, row, col, grid, rows, cols)
            on:click=move |_| cycle_status(row, col, grid)
            autocomplete="off"
        />
    }
}

#[component]
pub fn App() -> impl IntoView {
    let rows = RwSignal::new(3);
    let cols = RwSignal::new(5);

    let grid = RwSignal::new(
        (0..rows.get_untracked())
            .map(|_| {
                (0..cols.get_untracked())
                    .map(|_| RwSignal::new(Tile::default()))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<Vec<_>>>(),
    );

    // register reactive grid resize
    Effect::new(move || {
        update_grid_size(grid, rows.get(), cols.get());
    });

    let wordlist = get_str_asset("data/enable1.txt").expect("wordlist load failed");

    let is_solvable = Memo::new(move |_| is_grid_solvable(grid));

    let all_solutions = RwSignal::new(Vec::<&str>::new());
    let display_count = RwSignal::new(20);

    let solve = move || {
        solve_action(grid, cols, wordlist, all_solutions, display_count);
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
                        prop:value=move || rows.get()
                        min="1"
                        max="100"
                        on:change=move |e| {
                            if let Ok(val) = event_target_value(&e).parse::<usize>() {
                                if (1..=100).contains(&val) {
                                    rows.set(val);
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
                        prop:value=move || cols.get()
                        min="2"
                        // ethylenediaminetetraacetates
                        max="28"
                        on:change=move |e| {
                            if let Ok(val) = event_target_value(&e).parse::<usize>() {
                                if (2..=28).contains(&val) {
                                    cols.set(val);
                                }
                            }
                        }
                    />
                </label>
            </div>

            <div
                class="grid gap-2"
                style:grid-template-columns=move || {
                    format!("repeat({}, minmax(0, 1fr))", cols.get())
                }
            >
                {move || {
                    (0..rows.get())
                        .flat_map(|y| {
                            (0..cols.get()).map(move |x| { get_tile_view(y, x, grid, rows, cols) })
                        })
                        .collect_view()
                }}
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
