mod game;
mod solver;
use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use game::Game;
use solver::Solver;

fn load_words() -> Vec<String> {
    let f = File::open("wordlist.txt").expect("Could not open file");
    let reader = BufReader::new(f);
    let words: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    words
}

fn load_response() -> Vec<u8> {
    let mut ans: Vec<u8> = Vec::new();
    let mut inputs = String::new();
    io::stdin().read_line(&mut inputs).unwrap();
    for i in inputs.split_whitespace() {
        let state = parse_input!(i, u8);
        ans.push(state);
    }

    ans
}

fn main() {
    let words = load_words();

    let mut solver = Solver::new(words);

    let mut guess = String::new();

    loop {
        let response = load_response();

        if !response.iter().all(|&x| x == 0) {
            solver.filter_words(&guess, &response);
            guess = solver.pick_word();
        } else {
            guess = String::from("CARIES");
        }

        println!("{}", guess);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_words() {
        let words = load_words();
        assert!(!words.is_empty());
    }

    #[test]
    fn test_opener() {
        // very slow test of 12000s - result is CARIES
        let words = load_words();
        let solver = Solver::new(words.clone());
        let total = words.len() as f64;
        let mut best_score = 9999.0;
        let mut best_opener = String::new();
        for opener in &words {
            let mut score = 0.0;
            for target in &words {
                let game = Game {
                    current_word: target.clone(),
                };
                let response = game.check_guess(opener);

                let count = words
                    .iter()
                    .filter(|word| solver.check_word(word, opener, &response))
                    .count();

                if count == 0 {
                    continue;
                }
                let probability = 1.0 - (count as f64 / total);
                score += probability * -probability.log2();

                if score > best_score {
                    eprintln!("Opener: {}, Early stop: {}", opener, score);
                    break;
                }
            }

            if score < best_score {
                best_score = score;
                best_opener = opener.clone();
                eprintln!("New best opener: {}, Score: {}", best_opener, best_score);
            }
        }

        eprintln!("Best opener: {}, Score: {}", best_opener, best_score);
    }
}
