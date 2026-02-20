use crate::logic::guess::{CharGuess, GuessType, Guesses};

pub struct Requirement {
    word_len: usize,                   // Word length
    min_counts: [usize; 26],           // Minimum occurrences of each letter
    exact_counts: [Option<usize>; 26], // Exact occurrences of each letter
    must_be: Vec<Option<u8>>,          // What the letter in this spot must be
    must_not_be: Vec<Vec<u8>>,         // What the letter in this spot must not be
}

impl From<Guesses> for Requirement {
    fn from(item: Guesses) -> Self {
        let word_len = item.word_len;
        let mut reqs = Self {
            word_len: word_len,
            min_counts: [0; 26],
            exact_counts: [None; 26],
            must_be: vec![None; word_len],
            must_not_be: vec![vec![]; word_len],
        };
        for chunk in item.val.chunks(word_len) {
            let mut chunk_min_counts = [0; 26];
            for (pos, guess) in chunk.iter().enumerate() {
                let CharGuess { color, char } = guess;
                let idx = (char - b'a') as usize;
                match color {
                    GuessType::Green => {
                        reqs.must_be[pos] = Some(*char);
                        chunk_min_counts[idx] += 1;
                    }
                    GuessType::Yellow => {
                        reqs.must_not_be[pos].push(*char);
                        chunk_min_counts[idx] += 1;
                    }
                    GuessType::Gray => {
                        reqs.must_not_be[pos].push(*char);
                        reqs.exact_counts[idx] = Some(chunk_min_counts[idx]);
                    }
                }
            }
            for i in 0..26 {
                reqs.min_counts[i] = reqs.min_counts[i].max(chunk_min_counts[i]);
            }
        }
        reqs
    }
}

impl Requirement {
    pub fn applies_to(&self, word: &str) -> bool {
        if word.len() != self.word_len {
            return false;
        }
        let mut char_counts = [0; 26];
        for (pos, char) in word.bytes().map(|b| b.to_ascii_lowercase()).enumerate() {
            if let Some(must_exist_char) = self.must_be[pos]
                && char != must_exist_char
            {
                return false;
            } else if self.must_not_be[pos].contains(&char) {
                return false;
            }
            char_counts[(char - b'a') as usize] += 1;
        }
        for i in 0..26 {
            let char_count = char_counts[i];
            if let Some(count) = self.exact_counts[i]
                && char_count != count
            {
                return false;
            } else if char_count < self.min_counts[i] {
                return false;
            }
        }
        true
    }

    pub fn filter_wordlist<'a>(&self, wordlist: &'a str) -> impl Iterator<Item = &'a str> {
        wordlist.lines().filter(|word| self.applies_to(word))
    }
}
