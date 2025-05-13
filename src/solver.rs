use crate::game::Game;
use rand::Rng;
use std::collections::HashMap;

pub struct Solver {
    pub words: Vec<String>,
}

impl Solver {
    pub fn new(words: Vec<String>) -> Solver {
        Solver { words }
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
        let time = std::time::Instant::now();
        let mut best_words: Vec<String> = vec![];
        let mut best_score: f64 = -1.0;

        let mut counter: HashMap<u32, u16> = HashMap::new();
        for target_word in &self.words {
            counter.clear();
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

            if score > best_score {
                best_score = score;
                best_words = vec![target_word.clone()];
            } else if score == best_score {
                best_words.push(target_word.clone())
            }
        }
        eprintln!(
            "Testing {} words in {}us",
            self.words.len(),
            time.elapsed().as_micros()
        );
        eprintln!("{} words have the same score", best_words.len());

        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..best_words.len());
        best_words[idx].clone()
    }

    fn response_to_int(&self, response: &[u8]) -> u32 {
        let mut hash = 0u32;
        for value in response {
            hash = hash.wrapping_add(*value as u32);
            hash = hash.wrapping_mul(10);
        }
        hash
    }

    pub fn check_word(&self, word: &String, guess: &str, response: &[u8]) -> bool {
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
                    if !(word.chars().any(|x| x == c) && word.chars().nth(i).unwrap() != c) {
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
