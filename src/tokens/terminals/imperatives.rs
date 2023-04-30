use strum_macros::{EnumString, EnumIter};

/// Terminal keywords that represent actions that players must do when called.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(EnumString, EnumIter)]
#[strum(ascii_case_insensitive)]
pub enum ImperativeKW {
    Put,
    On,
    Return,
    From,
    To,
    Deals,
    Damage,
    Sacrifice,
}
