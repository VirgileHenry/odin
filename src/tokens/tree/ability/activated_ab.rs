use std::fmt;

use crate::{ability_display::AbilityDisplay, tokens::tree::{cost::Cost, statement::Statement}, ability_display_elems, ability_display_vec};

/// Represent an activated ability.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActivatedAbility {
    CostStatement(Vec<Cost>, Statement),
}

impl AbilityDisplay for ActivatedAbility {
    fn display(&self, f: &mut fmt::Formatter<'_>, padding: &mut Vec<bool>) -> fmt::Result {
        match self {
            ActivatedAbility::CostStatement(costs, stmt) => {
                write!(f, "Activated ability:\n")?;
                // have to do some stuff manually here
                for is_bar in padding.iter() {
                    if *is_bar { write!(f, "|   ")?; }
                    else { write!(f, "    ")?; }
                }
                write!(f, "|- Costs\n")?;
                padding.push(true);
                ability_display_vec!(f; padding; costs);
                padding.pop();
                write!(f, "\n")?;
                ability_display_elems!(f; padding; stmt);
            }
        }
        Ok(())
    }
}