use rand::Rng;

// Bonus added to the entropy of a guess that may be the answer: on equal information,
// prefer a word that can win right now (tuned by simulation over the whole word list).
const CANDIDATE_BONUS: f64 = 0.1;

pub struct Solver {
    pub words: Vec<String>,
    all_words: Vec<[u8; 6]>,
}

impl Solver {
    pub fn new(words: Vec<String>) -> Solver {
        let all_words = words.iter().map(|w| to_bytes(w)).collect();
        Solver { words, all_words }
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
        // With 1 or 2 candidates left, guessing one of them is optimal
        if self.words.len() <= 2 {
            return self.words[0].clone();
        }

        let time = std::time::Instant::now();
        let candidates: Vec<[u8; 6]> = self.words.iter().map(|w| to_bytes(w)).collect();
        let masks: Vec<u32> = candidates.iter().map(letter_mask).collect();
        let total = candidates.len() as f64;

        let mut best_words: Vec<[u8; 6]> = vec![];
        let mut best_score: f64 = -1.0;
        let mut counter = [0u16; 729];
        let mut touched: Vec<u16> = Vec::with_capacity(729);

        // Any word of the list can be guessed, not only the remaining candidates:
        // a non-candidate often splits the remaining words much better.
        for guess in &self.all_words {
            let mut is_candidate = false;
            for (target, mask) in candidates.iter().zip(&masks) {
                let hash = response_hash(guess, target, *mask);
                if hash == WIN_HASH {
                    is_candidate = true;
                }
                if counter[hash as usize] == 0 {
                    touched.push(hash);
                }
                counter[hash as usize] += 1;
            }

            let mut score: f64 = 0.0;
            for &hash in &touched {
                let probability = counter[hash as usize] as f64 / total;
                score += probability * -probability.log2();
                counter[hash as usize] = 0;
            }
            touched.clear();
            if is_candidate {
                score += CANDIDATE_BONUS;
            }

            if score > best_score {
                best_score = score;
                best_words = vec![*guess];
            } else if score == best_score {
                best_words.push(*guess)
            }
        }
        eprintln!(
            "Testing {} guesses on {} words in {}us",
            self.all_words.len(),
            self.words.len(),
            time.elapsed().as_micros()
        );
        eprintln!("{} words have the same score", best_words.len());

        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..best_words.len());
        String::from_utf8(best_words[idx].to_vec()).unwrap()
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

// Response encoded in base 3 (0 absent, 1 misplaced, 2 correct), same rules as Game::check_guess
const WIN_HASH: u16 = 728;

fn to_bytes(word: &str) -> [u8; 6] {
    word.as_bytes().try_into().expect("words must have 6 letters")
}

fn letter_mask(word: &[u8; 6]) -> u32 {
    word.iter().fold(0, |mask, &c| mask | 1 << (c - b'A'))
}

fn response_hash(guess: &[u8; 6], target: &[u8; 6], target_mask: u32) -> u16 {
    let mut hash = 0u16;
    for i in 0..6 {
        let c = guess[i];
        let value = if target[i] == c {
            2
        } else if target_mask >> (c - b'A') & 1 == 1 {
            1
        } else {
            0
        };
        hash = hash * 3 + value;
    }
    hash
}
