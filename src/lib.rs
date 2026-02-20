use wasm_bindgen::prelude::*;

use crate::{guess::Guesses, reqs::Requirement};

pub mod guess;
pub mod reqs;

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    let window = web_sys::window().expect("window expected");
    window.alert_with_message("Hello World from Rust!")?;
    Ok(())
}

#[wasm_bindgen]
pub fn solve(history: &str, wordlist: &str) -> Result<(), JsValue> {
    let guesses: Guesses = serde_json::from_str(history).unwrap();
    let reqs: Requirement = guesses.into();
    let window = web_sys::window().expect("window expected");
    for word in reqs.filter_wordlist(wordlist).take(10) {
        window.alert_with_message(word)?;
    }
    Ok(())
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}
