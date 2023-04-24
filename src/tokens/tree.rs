use std::fmt::{Display, self};
use std::str::FromStr;

use crate::ability_display_vec;
use crate::errors::OdinErrors;
use crate::lexer::lex;
use crate::parser::parse;
use crate::tokens::tree::ability::Ability;
use crate::ability_display::AbilityDisplay;


/// One or more ability.
/// This is the root of the Magic : the Gathering texts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbilityTree {
    pub abilities: Vec<Ability>
}

impl Display for AbilityTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut padding = vec![false];
        AbilityTree::display(&self, f, &mut padding)
    }
}

impl AbilityDisplay for AbilityTree {
    fn display(&self, f: &mut fmt::Formatter<'_>, padding: &mut Vec<bool>) -> fmt::Result {
        write!(f, "Abilities\n")?;
        ability_display_vec!(f; padding; self.abilities);
        Ok(())
    }
}

impl FromStr for AbilityTree {
    type Err = OdinErrors;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(parse(lex(input)?)?)
    }
}

pub mod ability;
pub mod tree_token;
pub mod imperative;
pub mod object_reference;
pub mod prefix_specifier;
pub mod suffix_specifier;
pub mod statement;
pub mod trigger_condition;
pub mod english_keywords;