use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use rand::distr::{Distribution, StandardUniform};
use rand::Rng;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Symbol {
    Rock,
    Paper,
    Scissors,
}

impl Symbol {
    /// Return a representative UTF-8 character.
    pub const fn as_char(self) -> char {
        match self {
            Self::Rock => '🪨',
            Self::Paper => '🧻',
            Self::Scissors => '✀',
        }
    }
}

impl PartialOrd for Symbol {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match (self, other) {
            (Self::Rock, Self::Rock)
            | (Self::Paper, Self::Paper)
            | (Self::Scissors, Self::Scissors) => Ordering::Equal,
            (Self::Rock, Self::Paper)
            | (Self::Paper, Self::Scissors)
            | (Self::Scissors, Self::Rock) => Ordering::Less,
            (Self::Paper, Self::Rock)
            | (Self::Scissors, Self::Paper)
            | (Self::Rock, Self::Scissors) => Ordering::Greater,
        })
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_char())
    }
}

impl Distribution<Symbol> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Symbol {
        match rng.random_range(0..=2) {
            0 => Symbol::Rock,
            1 => Symbol::Paper,
            2 => Symbol::Scissors,
            _ => unreachable!(),
        }
    }
}

impl FromStr for Symbol {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "r" | "🪨" => Ok(Self::Rock),
            "p" | "🧻" => Ok(Self::Paper),
            "s" | "✀" => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}
