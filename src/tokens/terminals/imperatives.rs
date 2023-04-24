use strum_macros::{EnumString, EnumIter};

/// Terminal keywords that represent actions that players must do when called.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(EnumString, EnumIter)]
#[strum(serialize_all = "snake_case")]
pub enum ImperativeKW {
    Put,
    On,
}
