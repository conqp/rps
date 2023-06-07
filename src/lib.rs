use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};

use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl Display for RPS {
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

impl PartialOrd<Self> for RPS {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Rock, Self::Paper) => Some(Ordering::Greater),
            (Self::Rock, Self::Scissors) => Some(Ordering::Less),
            (Self::Paper, Self::Rock) => Some(Ordering::Less),
            (Self::Paper, Self::Scissors) => Some(Ordering::Greater),
            (Self::Scissors, Self::Rock) => Some(Ordering::Greater),
            (Self::Scissors, Self::Paper) => Some(Ordering::Less),
            (_, _) => None,
        }
    }
}

impl Ord for RPS {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.partial_cmp(other) {
            Some(order) => order,
            None => Ordering::Equal,
        }
    }
}

impl Distribution<RPS> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RPS {
        match rng.gen_range(0..=2) {
            0 => RPS::Rock,
            1 => RPS::Paper,
            _ => RPS::Scissors,
        }
    }
}

impl FromStr for RPS {
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
