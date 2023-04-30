use crate::{tokens::terminals::{zone::Zone, specifiers::appartenace::AppartenanceSpecifier}, ability_display::AbilityDisplay, ability_display_elems};



/// Reference a zone.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZoneReference {
    TheBattlefield,
    OwnedZone(Zone, AppartenanceSpecifier),
}

impl AbilityDisplay for ZoneReference {
    fn display(&self, f: &mut std::fmt::Formatter<'_>, padding: &mut Vec<bool>) -> std::fmt::Result {
        match self {
            ZoneReference::TheBattlefield => write!(f, "The Battlefield")?,
            Self::OwnedZone(z, app) => {
                write!(f, "Owned zone:\n")?;
                ability_display_elems!(f; padding; app, z);
            }
        }
        Ok(())
    }
}