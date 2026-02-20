mod guess;
mod reqs;

use crate::{
    guess::{CharGuess, GuessType, Guesses},
    reqs::Requirement,
};

use leptos::prelude::*;
use wasm_bindgen::prelude::*;

fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    const WORDLIST_BYTES: &[u8] = include_bytes!("../assets/data/enable1.txt");
    let wordlist = unsafe { std::str::from_utf8_unchecked(WORDLIST_BYTES) };

    let history = Guesses {
        word_len: 8,
        val: vec![
            // Guess 1: ARTISTIC
            CharGuess {
                color: GuessType::Yellow,
                char: b'a',
            },
            CharGuess {
                color: GuessType::Gray,
                char: b'r',
            },
            CharGuess {
                color: GuessType::Gray,
                char: b't',
            },
            CharGuess {
                color: GuessType::Green,
                char: b'i',
            },
            CharGuess {
                color: GuessType::Gray,
                char: b's',
            },
            CharGuess {
                color: GuessType::Gray,
                char: b't',
            },
            CharGuess {
                color: GuessType::Yellow,
                char: b'i',
            },
            CharGuess {
                color: GuessType::Yellow,
                char: b'c',
            },
            // Guess 2: OFFICIAL
            CharGuess {
                color: GuessType::Gray,
                char: b'o',
            },
            CharGuess {
                color: GuessType::Gray,
                char: b'f',
            },
            CharGuess {
                color: GuessType::Gray,
                char: b'f',
            },
            CharGuess {
                color: GuessType::Green,
                char: b'i',
            },
            CharGuess {
                color: GuessType::Yellow,
                char: b'c',
            },
            CharGuess {
                color: GuessType::Green,
                char: b'i',
            },
            CharGuess {
                color: GuessType::Green,
                char: b'a',
            },
            CharGuess {
                color: GuessType::Yellow,
                char: b'l',
            },
        ],
    };

    let reqs: Requirement = history.into();
    let possible_word = reqs.filter_wordlist(wordlist).next().unwrap();

    leptos::mount::mount_to_body(move || {
        view! {
            <p>"Word: " {possible_word}</p>
        }
    });

    Ok(())
}
