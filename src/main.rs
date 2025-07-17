use std::cmp::Ordering;
use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};
use std::str::FromStr;

use rand::Rng;
use rps::{RoundResult, Symbol};

fn main() {
    let mut rounds: u8 = 0;
    let mut score: i8 = 0;
    let mut rng = rand::thread_rng();

    loop {
        if rounds >= 3 {
            break;
        }

        let computer: Symbol = rng.gen();
        let player = get_plyer_input();

        match player.compare(&computer) {
            RoundResult::Draw => {
                println!("Draw: {} = {}", player, computer);
                continue;
            }
            RoundResult::RightWins => {
                println!("You lost: {} < {}", player, computer);
                score -= 1;
            }
            RoundResult::LeftWins => {
                println!("You won: {} > {}", player, computer);
                score += 1;
            }
        }

        rounds += 1;
    }

    match score.cmp(&0) {
        Ordering::Equal => println!("The game ended in a draw."),
        Ordering::Less => println!("The computer won the game."),
        Ordering::Greater => println!("The player won the game."),
    }
}

fn get_plyer_input() -> Symbol {
    let mut stdout = BufWriter::new(stdout().lock());
    let mut stdin = BufReader::new(stdin().lock());

    loop {
        write!(stdout, "Your selection (r/p/s): ").expect("Could not write to STDOUT.");
        stdout.flush().expect("Could not flush STDOUT.");
        let mut buf = String::new();

        if stdin.read_line(&mut buf).is_ok() {
            match Symbol::from_str(&buf) {
                Ok(rps) => return rps,
                Err(error) => println!("{}: {}", error, buf),
            }
        }
    }
}
