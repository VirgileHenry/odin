pub trait Terminal: std::fmt::Display + Sized {
    fn repr(&self) -> &'static str;
    fn iter() -> impl Iterator<Item = Self>;
    fn try_from_str(source: &str) -> Option<(Self, usize)> {
        for item in Self::iter() {
            if source.starts_with(item.repr()) {
                let length = item.repr().len();
                return Some((item, length));
            }
        }
        None
    }
}

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

impl Terminal for Number {
    fn repr(&self) -> &'static str {
        unreachable!("repr() shall not be used for the \"Number\" terminal!");
    }
    fn iter() -> impl Iterator<Item = Self> {
        fn inner() -> std::iter::Empty<Number> {
            unreachable!("iter() shall not be used for the \"Number\" terminal!");
        }
        inner()
    }
    fn try_from_str(source: &str) -> Option<(Self, usize)> {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Counter {
    PlusOne,
}

impl std::fmt::Display for Counter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Counter::PlusOne => write!(f, "+1/+1 Counter"),
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
pub enum Appartenance {
    Your,
    AnyPlayer,
    AnOpponent,
}

impl std::fmt::Display for Appartenance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Appartenance::Your => write!(f, "Your"),
            Appartenance::AnyPlayer => write!(f, "Any Player's"),
            Appartenance::AnOpponent => write!(f, "An Opponent's"),
        }
    }
}

impl Terminal for Appartenance {}

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

impl Terminal for Actions {}
