use std::fs;

use wordle_solver::{
    guess::{CharGuess, GuessType, Guesses},
    reqs::Requirement,
};

fn main() {
    let wordlist = fs::read_to_string("assets/enable1.txt").unwrap();
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
    let serialized = serde_json::to_string(&history).unwrap();
    println!("{}", serialized);
    let reqs: Requirement = history.into();
    for word in reqs.filter_wordlist(&wordlist).take(100) {
        println!("{}", word);
    }
}
