use strum_macros::{EnumString, EnumIter};

/// Terminal keywords that represent punctuation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(EnumString, EnumIter)]
#[strum(ascii_case_insensitive)]
pub enum ControlFlow {
    #[strum(serialize = "\n")]
    NewLine,
    #[strum(serialize = ",")]
    Comma,
    #[strum(serialize = ".")]
    Dot,
    #[strum(serialize = ":")]
    Colons,
}
