pub trait Terminal: std::fmt::Display {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Number {
    A,
    Number(i32),
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::A => write!(f, "A"),
            Number::Number(num) => write!(f, "{num}"),
        }
    }
}

impl Terminal for Number {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Counter {
    PlusOne,
}

impl std::fmt::Display for Counter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Counter::PlusOne => write!(f, "+1/+1"),
        }
    }
}

impl Terminal for Counter {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CountSpecifier {
    Each,
    Target,
    UpTo(usize),
}

impl std::fmt::Display for CountSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CountSpecifier::Each => write!(f, "Each"),
            CountSpecifier::Target => write!(f, "Target"),
            CountSpecifier::UpTo(amount) => write!(f, "Up To {amount}"),
        }
    }
}

impl Terminal for CountSpecifier {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ControlSpecifier {
    YouControl,
    YouDontControl,
}

impl std::fmt::Display for ControlSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ControlSpecifier::YouControl => write!(f, "You Control"),
            ControlSpecifier::YouDontControl => write!(f, "You Don't Control"),
        }
    }
}

impl Terminal for ControlSpecifier {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AppartenanceSpecifier {
    Your,
    AnyPlayer,
    AnOpponent,
}

impl std::fmt::Display for AppartenanceSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppartenanceSpecifier::Your => write!(f, "Your"),
            AppartenanceSpecifier::AnyPlayer => write!(f, "Any Player's"),
            AppartenanceSpecifier::AnOpponent => write!(f, "An Opponent's"),
        }
    }
}

impl Terminal for AppartenanceSpecifier {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Actions {
    Dies,
    Attacks,
}

impl std::fmt::Display for Actions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Actions::Dies => write!(f, "Dies"),
            Actions::Attacks => write!(f, "Attacks"),
        }
    }
}
