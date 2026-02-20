use crate::logic::{guess::Guesses, reqs::Requirement};

use std::path::Path;

use include_dir::{Dir, include_dir};
use leptos::prelude::*;

static ASSETS: Dir = include_dir!("$CARGO_MANIFEST_DIR/assets");

fn get_utf8_assets<S>(path: S) -> Option<&'static str>
where
    S: AsRef<Path>,
{
    let bytes = ASSETS.get_file(path)?.contents();
    std::str::from_utf8(bytes).ok()
}

#[component]
pub fn App() -> impl IntoView {
    let wordlist = get_utf8_assets("data/enable1.txt").expect("wordlist load failed");

    let hardcoded_guesses_string = get_utf8_assets("data/guesses.json").expect("hardcoded guesses load failed");
    let hardcoded_guesses_struct: Guesses =
        serde_json::from_str(hardcoded_guesses_string).expect("hardcoded guessses JSON parse failed");
    let reqs: Requirement = hardcoded_guesses_struct.into();

    let word = reqs.filter_wordlist(wordlist).next().expect("no possible words found");

    view! { <p>"Word: " {word}</p> }
}
