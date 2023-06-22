use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RoundResult {
    Draw,
    LeftWins,
    RightWins,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Symbol {
    Rock,
    Paper,
    Scissors,
}

impl Symbol {
    pub fn compare(&self, other: &Self) -> RoundResult {
        if self.wins_against(other) {
            RoundResult::LeftWins
        } else if other.wins_against(self) {
            RoundResult::RightWins
        } else {
            RoundResult::Draw
        }
    }
    pub fn wins_against(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::Rock, Self::Scissors)
                | (Self::Paper, Self::Rock)
                | (Self::Scissors, Self::Paper)
        )
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Rock => '🪨',
                Self::Paper => '🧻',
                Self::Scissors => '✀',
            }
        )
    }
}

impl Distribution<Symbol> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Symbol {
        match rng.gen_range(0..=2) {
            0 => Symbol::Rock,
            1 => Symbol::Paper,
            _ => Symbol::Scissors,
        }
    }
}

impl FromStr for Symbol {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "r" | "🪨" => Ok(Self::Rock),
            "p" | "🧻" => Ok(Self::Paper),
            "s" | "✀" => Ok(Self::Scissors),
            _ => Err("invalid string"),
        }
    }
}
