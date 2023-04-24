use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;
use strum_macros::EnumIter;

use crate::ability_display::AbilityDisplay;

// const regex to match numbers
lazy_static!{
    static ref NUMBER_REGEX: Regex = Regex::new("[0-9]+").unwrap(); // ! fixme
}

/// Terminal keywords that represent numbers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(EnumIter)]
pub enum Number {
    A,
    Number(usize),
}

impl FromStr for Number {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "a" {
            return Ok(Number::A);
        }
        // todo : waiting for fix from regex
        /*
        if NUMBER_REGEX.is_match(s) {
            match usize::from_str(s) {
                Ok(n) => return Ok(Number::Number(n)),
                Err(_) => {},
            }
        }
        */
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