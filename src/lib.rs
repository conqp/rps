use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Symbol {
    Rock,
    Paper,
    Scissors,
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

impl PartialOrd<Self> for Symbol {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Rock, Self::Paper) => Some(Ordering::Less),
            (Self::Rock, Self::Scissors) => Some(Ordering::Greater),
            (Self::Paper, Self::Rock) => Some(Ordering::Greater),
            (Self::Paper, Self::Scissors) => Some(Ordering::Less),
            (Self::Scissors, Self::Rock) => Some(Ordering::Less),
            (Self::Scissors, Self::Paper) => Some(Ordering::Greater),
            (_, _) => None,
        }
    }
}

impl Ord for Symbol {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.partial_cmp(other) {
            Some(order) => order,
            None => Ordering::Equal,
        }
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
            "r" => Ok(Self::Rock),
            "p" => Ok(Self::Paper),
            "s" => Ok(Self::Scissors),
            _ => Err("invalid string"),
        }
    }
}
