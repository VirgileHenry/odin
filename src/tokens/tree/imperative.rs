use crate::{ability_display::AbilityDisplay, ability_display_elems, tokens::terminals::{numbers::Number, counters::Counter}};

use super::{object_reference::ObjectReference, zone_reference::ZoneReference};

/// An imperative action that a player must take when called.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Imperative {
    /// The "put" imperative action.
    Put(Number, Counter, ObjectReference),
    Return(ObjectReference, ZoneReference, ZoneReference),
    DealsDamage(ObjectReference, Number, ObjectReference),
    Sacrifice(ObjectReference),
}


impl AbilityDisplay for Imperative {
    fn display(&self, f: &mut std::fmt::Formatter<'_>, padding: &mut Vec<bool>) -> std::fmt::Result {
        match self {
            Imperative::Put(n, c, o) => {
                write!(f, "Put counters on object:\n")?;
                ability_display_elems!(f; padding; n, c, o);
            },
            Imperative::Return(obj, z1, z2) => {
                write!(f, "Return object from zone to zone:\n")?;
                ability_display_elems!(f; padding; obj, z1, z2);
            },
            Imperative::DealsDamage(obj1, num, obj2) => {
                write!(f, "Deals damages to:\n")?;
                ability_display_elems!(f; padding; obj1, num, obj2);
            },
            Imperative::Sacrifice(obj) => {
                write!(f, "Sacrifice:\n")?;
                ability_display_elems!(f; padding; obj);
            },
        }
        Ok(())
    }
}
