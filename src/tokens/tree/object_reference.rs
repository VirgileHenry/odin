use crate::{tokens::terminals::{objects::Object, specifiers::count::CountSpecifier}, ability_display::AbilityDisplay, ability_display_elems};

use super::{prefix_specifier::ObjectPrefixSpecifier, suffix_specifier::ObjectSuffixSpecifier};

/// A reference to an object. 
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObjectReference {
    /// Reference the card or object that possess the ability. 
    SelfReferencing,
    /// Reference a number of objects, with prefix and suffix specifiers.
    SpecifiedObj(CountSpecifier, Object, Vec<ObjectPrefixSpecifier>, Vec<ObjectSuffixSpecifier>),
}

impl AbilityDisplay for ObjectReference {
    fn display(&self, f: &mut std::fmt::Formatter<'_>, padding: &mut Vec<bool>) -> std::fmt::Result {
        match self {
            ObjectReference::SelfReferencing => {
                write!(f, "Self Referencing")?
            },
            ObjectReference::SpecifiedObj(c, o, p, s) => {
                write!(f, "Specified Object(s):\n")?;
                match (p.is_empty(), s.is_empty()) {
                    (false, false) => ability_display_elems!(f; padding; c, p, o, s),
                    (true, false) => ability_display_elems!(f; padding; c, o, s),
                    (false, true) => ability_display_elems!(f; padding; c, p, o),
                    (true, true) => ability_display_elems!(f; padding; c, o),
                }
            },
        }
        Ok(())
    }
}