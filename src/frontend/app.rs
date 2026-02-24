use include_dir::{Dir, include_dir};
use leptos::prelude::*;
use std::path::Path;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, KeyboardEvent};

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

#[derive(Clone, PartialEq)]
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
    let cursor = RwSignal::new(0);
    let all_solutions = RwSignal::new(Vec::<&'static str>::new());
    let solved = RwSignal::new(false);

    let wordlist = get_str_asset("data/enable1.txt").expect("wordlist load failed");

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

    let solve = move || {
        let guesses = grid.with(|g| Guesses::try_from(g).expect("tiles should be filled"));
        let reqs: Requirement = guesses.into();
        let results: Vec<&str> = reqs.filter_wordlist(wordlist).collect();
        all_solutions.set(results);
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

        <Solutions solved=solved all_solutions=all_solutions />
      </main>
    }
}

#[component]
fn NumberCounter(
    value: Signal<usize>,
    #[prop(into)] on_change: Callback<usize>,
    #[prop(default = 1)] min: usize,
) -> impl IntoView {
    view! {
      <div class="flex items-center border-2 border-black">
        <button
          class="px-3 py-1 font-bold bg-white hover:bg-black hover:text-white transition-colors select-none active:scale-95"
          on:click=move |_| {
            let v = value.get();
            if v > min {
              on_change.run(v - 1);
            }
          }
        >
          "-"
        </button>
        <span class="w-8 text-center font-bold">{move || value.get()}</span>
        <button
          class="px-3 py-1 font-bold bg-white hover:bg-black hover:text-white transition-colors select-none active:scale-95"
          on:click=move |_| on_change.run(value.get() + 1)
        >
          "+"
        </button>
      </div>
    }
}

#[component]
fn Settings(
    grid: RwSignal<Grid>,
    #[prop(into)] resize_grid: Callback<(Option<usize>, Option<usize>)>,
) -> impl IntoView {
    view! {
      <div class="flex items-center gap-6">
        <div class="flex items-center gap-2">
          "Guess count: "
          <NumberCounter
            value=Signal::derive(move || grid.read().rows)
            on_change=move |v| resize_grid.run((Some(v), None))
          />
        </div>
        <div class="flex items-center gap-2">
          "Word length: "
          <NumberCounter
            value=Signal::derive(move || grid.read().cols)
            on_change=move |v| resize_grid.run((None, Some(v)))
          />
        </div>
      </div>
    }
}

#[component]
fn Grid(grid: RwSignal<Grid>) -> impl IntoView {
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

#[component]
fn SolveButton(#[prop(into)] solve: Callback<()>, is_solvable: Memo<bool>) -> impl IntoView {
    view! {
      <div>
        <button
          class="border-2 border-black p-2 px-6 font-bold uppercase bg-white hover:bg-black hover:text-white transition-colors h-[46px] disabled:opacity-50 active:scale-95"
          on:click=move |_| solve.run(())
          disabled=move || !is_solvable.get()
        >
          "Solve"
        </button>
      </div>
    }
}

#[component]
fn Solutions(solved: RwSignal<bool>, all_solutions: RwSignal<Vec<&'static str>>) -> impl IntoView {
    let scroll_ref = NodeRef::<leptos::html::Div>::new();
    let count = Memo::new(move |_| all_solutions.read().len());

    view! {
      <Show when=move || solved.get()>
        <div class="w-full max-w-2xl px-4">
          <p class="text-sm font-semibold mb-2">
            {move || {
              let count = all_solutions.with(|s| s.len());
              format!("{} solution{} found", count, if count == 1 { "" } else { "s" })
            }}
          </p>

          <Show
            when=move || { count.get() > 0 }
            fallback=|| {
              view! {
                <p class="text-sm text-gray-500 italic">
                  "No matching words found. Check for contradictions."
                </p>
              }
            }
          >

            <div
              node_ref=scroll_ref
              class="overflow-y-auto max-h-[40vh] border-2 border-black bg-white"
            >

              <div class="flex flex-wrap gap-2 p-3 justify-center">
                <For
                  each=move || { all_solutions.get().into_iter() }
                  key=|word| *word
                  children=|word| {
                    view! {
                      <span class="bg-gray-200 px-3 py-1 text-sm font-semibold tracking-wider uppercase">
                        {word}
                      </span>
                    }
                  }
                />
              </div>
            </div>
          </Show>
        </div>
      </Show>
    }
}
