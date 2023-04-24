use crate::{tokens::{terminals::{numbers::Number, counters::Counter}, tree::object_reference::ObjectReference}, ability_display::AbilityDisplay, ability_display_elems};

/// The imperative "Put" and all it's different meanings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Put {
    /// Put a number of counters on something.
    Counters(Number, Counter, ObjectReference),
}

impl AbilityDisplay for Put {
    fn display(&self, f: &mut std::fmt::Formatter<'_>, padding: &mut Vec<bool>) -> std::fmt::Result {
        match self {
            Put::Counters(n, c, o) => {
                write!(f, "Put Counters:\n")?;
                ability_display_elems!(f; padding; n, c, o);
            }
        }
        Ok(())
    }
}