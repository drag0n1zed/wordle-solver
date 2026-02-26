mod grid;
mod settings;
mod solutions;
mod wordlist;

use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, KeyboardEvent};

use crate::backend::{
    guess::{GuessColor, Guesses},
    reqs::Requirement,
};

use grid::{Grid, Tile};
use settings::Settings;
use solutions::{SolutionEntry, Solutions, SolveButton};
use wordlist::WordlistSource;

#[component]
pub fn App() -> impl IntoView {
    let grid = RwSignal::new(Grid::new(3, 5));
    let cursor = RwSignal::new(0);
    let solutions: RwSignal<Vec<SolutionEntry>> = RwSignal::new(Vec::new());
    let solved = RwSignal::new(false);

    let wordlists = RwSignal::new(vec![
        wordlist::fetch("Common Words", WordlistSource::Url("/assets/popular.txt"), false),
        wordlist::fetch("All Words", WordlistSource::Url("/assets/enable1.txt"), false),
    ]);

    let resize_grid = move |new_rows: Option<usize>, new_cols: Option<usize>| {
        grid.update(|g| {
            if let Some(r) = new_rows {
                g.rows = r;
            }
            if let Some(c) = new_cols {
                g.cols = c;
                g.tiles = vec![vec![Tile::default(); g.cols]; g.rows];
                cursor.set(0);
            }
            g.tiles.resize(g.rows, vec![Tile::default(); g.cols]);
            for row in g.tiles.iter_mut() {
                row.resize(g.cols, Tile::default());
            }
        });
        if new_cols.is_none() {
            let new_total = grid.with_untracked(|g| g.rows * g.cols);
            cursor.update(|c| *c = new_total.min(*c));
        }
    };

    let solve_counter = RwSignal::new(0);

    let solve = move || {
        let guesses = grid.with(|g| Guesses::try_from(g).expect("tiles should be filled"));
        let reqs: Requirement = guesses.into();

        let id = solve_counter.get();
        solve_counter.update(|c| *c += 1);

        let results = wordlists.with(|wls| {
            wls.iter()
                .map(|wls| {
                    let mut entry = wls.filter(&reqs);
                    entry.id = id;
                    entry
                })
                .collect()
        });
        solutions.set(results);
        solved.set(true);
    };

    let is_solvable =
        Memo::new(move |_| grid.with(|g| g.tiles.iter().flatten().all(|t| t.char.is_some() && t.color.is_some())));

    window_event_listener(leptos::ev::keydown, move |e: KeyboardEvent| {
        if document()
            .active_element()
            .and_then(|el| el.dyn_into::<HtmlInputElement>().ok())
            .is_some()
        {
            return;
        }

        match e.key().as_str() {
            "Backspace" => {
                e.prevent_default();
                let pos = cursor.get_untracked();
                if pos > 0 {
                    let new_pos = pos - 1;
                    grid.update(|g| {
                        let t = &mut g.tiles[new_pos / g.cols][new_pos % g.cols];
                        t.char = None;
                        t.color = None;
                    });
                    cursor.set(new_pos);
                }
            }
            k if k.len() == 1 && k.chars().next().unwrap().is_ascii_alphabetic() => {
                let char = k.chars().next().unwrap().to_ascii_uppercase() as u8;
                let total = grid.with_untracked(|g| g.rows * g.cols);
                let pos = cursor.get_untracked();
                let write_pos = pos.min(total - 1);

                grid.update(|g| {
                    let t = &mut g.tiles[write_pos / g.cols][write_pos % g.cols];
                    if t.char.is_none() {
                        t.color = Some(GuessColor::Gray);
                    }
                    t.char = Some(char);
                });
                if pos < total {
                    cursor.set(pos + 1);
                }
            }
            _ => {}
        }
    });

    view! {
      <main class="flex flex-col items-center min-h-screen gap-6 py-8">

        <div class="h-[10vh]"></div>

        <Settings grid=grid resize_grid=resize_grid />

        <Grid grid=grid />

        <SolveButton solve=solve is_solvable=is_solvable />

        <Solutions solved=solved solutions=solutions />
      </main>
    }
}
