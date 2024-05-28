mod game;
mod solver;

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

fn main() {
    let words = load_words();
    let game = Game::new(&words);
    let mut solver = Solver::new(words);

    loop {
        let guess = solver.pick_word();
        let response = game.check_guess(&guess);
        eprintln!("Guess: {}, Response: {:?}", guess, response);

        if response.iter().all(|&x| x == 3) {
            eprintln!("Found word: {}", guess);
            break;
        }

        solver.filter_words(&guess, &response);
    }
}
