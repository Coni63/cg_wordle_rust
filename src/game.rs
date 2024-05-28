use rand::Rng;

pub struct Game {
    pub current_word: String,
}

impl Game {
    pub fn new(words: &[String]) -> Game {
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..words.len());
        let current_word = words[idx].clone();
        eprintln!("current_word: {}", current_word);
        Game { current_word }
    }

    pub fn check_guess(&self, guess: &str) -> Vec<u8> {
        let mut ans = Vec::new();
        for (i, c) in guess.chars().enumerate() {
            if self.current_word.chars().nth(i).unwrap() == c {
                ans.push(3u8);
            } else if self.current_word.chars().any(|x| x == c) {
                ans.push(2u8);
            } else {
                ans.push(1u8);
            }
        }
        ans
    }
}
