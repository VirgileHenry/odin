use std::str::FromStr;

use strum_macros::EnumIter;

use crate::ability_display::AbilityDisplay;

/// Terminal keywords that represent numbers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(EnumIter)]
pub enum Number {
    A,
    Number(u32),
}

impl FromStr for Number {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "a" {
            return Ok(Number::A);
        }
        Err(())
    }
}

impl AbilityDisplay for Number {
    fn display(&self, f: &mut std::fmt::Formatter<'_>, _padding: &mut Vec<bool>) -> std::fmt::Result {
        match self {
            Number::A => write!(f, "A"),
            Number::Number(n) => write!(f, "Number: {n}"),
        }
    }
}