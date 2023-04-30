use mtg_data::MtgColor;

use crate::{ability_display::AbilityDisplay, ability_display_vec, tokens::terminals::{specifiers::appartenace::AppartenanceSpecifier, objects::Object}};

/// An object reference specifier that is before the object.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ObjectPrefixSpecifier {
    /// A specifier for the color of the Object.
    Color(MtgColor),
    Object(Object),
}

impl AbilityDisplay for ObjectPrefixSpecifier {
    fn display(&self, f: &mut std::fmt::Formatter<'_>, _padding: &mut Vec<bool>) -> std::fmt::Result {
        match self {
            Self::Color(c) => write!(f, "Color: {c:?}")?,
            Self::Object(o) => write!(f, "Object (as type specifier): {o:?}")?,
        }
        Ok(())
    }
}

impl AbilityDisplay for Vec<ObjectPrefixSpecifier> {
    fn display(&self, f: &mut std::fmt::Formatter<'_>, padding: &mut Vec<bool>) -> std::fmt::Result {
        write!(f, "Prefixes:\n")?;
        ability_display_vec!(f; padding; self);
        Ok(())
    }
}

/// a zone specifier.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ZonePrefixSpecifier {
    Appartenance(AppartenanceSpecifier),
}

impl AbilityDisplay for ZonePrefixSpecifier {
    fn display(&self, f: &mut std::fmt::Formatter<'_>, _padding: &mut Vec<bool>) -> std::fmt::Result {
        match self {
            Self::Appartenance(a) => write!(f, "Color: {a:?}")?,
        }
        Ok(())
    }
}

impl AbilityDisplay for Vec<ZonePrefixSpecifier> {
    fn display(&self, f: &mut std::fmt::Formatter<'_>, padding: &mut Vec<bool>) -> std::fmt::Result {
        write!(f, "Prefixes:\n")?;
        ability_display_vec!(f; padding; self);
        Ok(())
    }
}