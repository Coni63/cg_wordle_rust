use std::collections::HashMap;

use crate::game::Game;

pub struct Solver {
    pub words: Vec<String>,
    pub possibilities: [[bool; 26]; 6],
}

impl Solver {
    pub fn new(words: Vec<String>) -> Solver {
        Solver {
            words,
            possibilities: [[true; 26]; 6],
        }
    }

    pub fn filter_words(&mut self, guess: &str, response: &[u8]) {
        let choices_start = self.words.len();
        let time = std::time::Instant::now();
        self.words = self
            .words
            .iter()
            .filter(|word| self.check_word(word, guess, response))
            .cloned()
            .collect();
        let choices_end = self.words.len();
        eprintln!(
            "Reduced to {} -> {} in {}",
            choices_start,
            choices_end,
            time.elapsed().as_micros()
        );
    }

    pub fn pick_word(&self) -> String {
        if self.words.len() > 2000 {
            return String::from("CARIES");
        }

        let mut best_word = String::new();
        let mut best_score: f64 = 0.0;

        for target_word in &self.words {
            let mut counter: HashMap<u32, u16> = HashMap::new();
            let test_game = Game {
                current_word: target_word.clone(),
            };

            for word in &self.words {
                let response = test_game.check_guess(word);
                let hash = self.response_to_int(&response);
                *counter.entry(hash).or_insert(0) += 1;
            }

            let total = self.words.len() as f64;
            let mut score: f64 = 0.0;

            for count in counter.values() {
                let probability = *count as f64 / total;
                score += probability * -probability.log2();
            }

            // eprintln!("{}: {}", target_word, score);

            if score >= best_score {
                best_score = score;
                best_word = target_word.clone();
            }
        }

        best_word
    }

    fn response_to_int(&self, response: &[u8]) -> u32 {
        let mut hash = 0u32;
        for value in response {
            hash = hash.wrapping_add(*value as u32);
            hash = hash.wrapping_mul(10);
        }
        hash
    }

    fn check_word(&self, word: &String, guess: &str, response: &[u8]) -> bool {
        if word == &guess.to_string() {
            // This line is added to prevent the same word from being guessed again
            return false;
        }

        for (i, c) in guess.chars().enumerate() {
            match response[i] {
                3 => {
                    if word.chars().nth(i).unwrap() != c {
                        return false;
                    }
                }
                2 => {
                    if !word.chars().any(|x| x == c) {
                        return false;
                    }
                }
                1 => {
                    if word.chars().any(|x| x == c) {
                        return false;
                    }
                }
                _ => {
                    return true;
                }
            }
        }
        true
    }
}
