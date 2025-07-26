//! A Rock-Paper-Scissors game.

use std::cmp::Ordering;
use std::io::{stdin, stdout, BufRead, Write};

use rand::Rng;
use symbol::Symbol;

mod symbol;

const ROUNDS: usize = 3;

fn main() {
    let mut score: i8 = 0;
    let mut rng = rand::rng();
    let mut buf = String::new();

    for _ in 1..=ROUNDS {
        let computer: Symbol = rng.random();
        let player = get_player_input(&mut buf);

        match player.partial_cmp(&computer) {
            Some(Ordering::Equal) => {
                println!("Draw: {player} = {computer}");
            }
            Some(Ordering::Less) => {
                println!("You lost: {player} < {computer}");
                score -= 1;
            }
            Some(Ordering::Greater) => {
                println!("You won: {player} > {computer}");
                score += 1;
            }
            _ => unreachable!(),
        }
    }

    match score.cmp(&0) {
        Ordering::Equal => println!("The game ended in a draw."),
        Ordering::Less => println!("The computer won the game."),
        Ordering::Greater => println!("The player won the game."),
    }
}

fn get_player_input(buf: &mut String) -> Symbol {
    let mut stdout = stdout().lock();
    let mut stdin = stdin().lock();

    loop {
        write!(stdout, "Your selection (r/p/s): ").expect("Could not write to STDOUT.");
        stdout.flush().expect("Could not flush STDOUT.");
        buf.clear();

        if stdin.read_line(buf).is_ok() {
            match buf.trim().to_lowercase().parse() {
                Ok(rps) => return rps,
                Err(()) => println!("Invalid input: {buf}"),
            }
        }
    }
}
