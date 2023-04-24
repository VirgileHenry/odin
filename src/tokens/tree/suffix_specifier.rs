use crate::{tokens::terminals::specifiers::control::ControlSpecifier, ability_display::AbilityDisplay, ability_display_vec};


/// An object reference specifier, that is after the object itself.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ObjectSuffixSpecifier {
    /// A specifier for the control of the object.
    Control(ControlSpecifier),
}

impl AbilityDisplay for ObjectSuffixSpecifier {
    fn display(&self, f: &mut std::fmt::Formatter<'_>, padding: &mut Vec<bool>) -> std::fmt::Result {
        match self {
            Self::Control(c) => {
                write!(f, "Control: ")?;
                // control specifiers have a custom writing, redirect
                c.display(f, padding)?;
            }
        }
        Ok(())
    }
}

impl AbilityDisplay for Vec<ObjectSuffixSpecifier> {
    fn display(&self, f: &mut std::fmt::Formatter<'_>, padding: &mut Vec<bool>) -> std::fmt::Result {
        write!(f, "Suffixes:\n")?;
        ability_display_vec!(f; padding; self);
        Ok(())
    }
}