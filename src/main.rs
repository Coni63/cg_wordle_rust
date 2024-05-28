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

    let mut distrib = [0; 10];

    for _ in 0..10 {
        let game = Game::new(&words);
        let mut solver = Solver::new(words.clone());

        let mut count = 0;
        loop {
            let guess = solver.pick_word();
            let response = game.check_guess(&guess);
            count += 1;
            // eprintln!("Guess: {}, Response: {:?}", guess, response);

            if response.iter().all(|&x| x == 3) {
                eprintln!("Found {} word in {}", guess, count);
                distrib[count] += 1;
                break;
            }

            solver.filter_words(&guess, &response);
        }
    }

    eprintln!("{:?}", distrib);
}
