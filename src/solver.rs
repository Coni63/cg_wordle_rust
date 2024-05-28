use rand::Rng;

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
        self.words = self
            .words
            .iter()
            .filter(|word| self.check_word(word, guess, response))
            .cloned()
            .collect();
        let choices_end = self.words.len();
        eprintln!("Reduced to {} -> {}", choices_end, choices_start);
    }

    pub fn pick_word(&self) -> String {
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..self.words.len());
        self.words[idx].clone()
    }

    fn check_word(&self, word: &String, guess: &str, response: &[u8]) -> bool {
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
